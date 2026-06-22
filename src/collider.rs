use crate::Vec2;
use crate::bodies::{Body, Shape};

struct Collision {
    normal: Vec2,
    depth: f32,
    contacts: Vec<Vec2>,
}

fn get_corners(pos: Vec2, rot: f32, w: f32, h: f32) -> [Vec2; 4] {
    let (sin, cos) = rot.sin_cos();
    let hw = w / 2.0;
    let hh = h / 2.0;
    [
        pos + Vec2::new(-hw * cos + hh * sin, -hw * sin - hh * cos),
        pos + Vec2::new(hw * cos + hh * sin, hw * sin - hh * cos),
        pos + Vec2::new(hw * cos - hh * sin, hw * sin + hh * cos),
        pos + Vec2::new(-hw * cos - hh * sin, -hw * sin + hh * cos),
    ]
}

fn sat_overlap(axis: Vec2, ca: &[Vec2; 4], cb: &[Vec2; 4]) -> Option<f32> {
    let project = |corners: &[Vec2; 4]| {
        let dots: [f32; 4] = corners.map(|c| c.dot(axis));
        (
            dots.iter().cloned().fold(f32::INFINITY, f32::min),
            dots.iter().cloned().fold(f32::NEG_INFINITY, f32::max),
        )
    };
    let (a_min, a_max) = project(ca);
    let (b_min, b_max) = project(cb);
    let o1 = a_max - b_min;
    let o2 = b_max - a_min;
    if o1 < 0.0 || o2 < 0.0 {
        None
    } else {
        Some(o1.min(o2))
    }
}

fn box_axes(rot: f32) -> [Vec2; 2] {
    let (sin, cos) = rot.sin_cos();
    [Vec2::new(cos, sin), Vec2::new(-sin, cos)]
}

fn best_face(pos: Vec2, rot: f32, w: f32, h: f32, dir: Vec2) -> (Vec2, Vec2, Vec2) {
    let [xa, ya] = box_axes(rot);
    let hw = w / 2.0;
    let hh = h / 2.0;
    let faces = [
        (xa, pos + xa * hw - ya * hh, pos + xa * hw + ya * hh),
        (-xa, pos - xa * hw + ya * hh, pos - xa * hw - ya * hh),
        (ya, pos - xa * hw + ya * hh, pos + xa * hw + ya * hh),
        (-ya, pos + xa * hw - ya * hh, pos - xa * hw - ya * hh),
    ];
    let &(n, v0, v1) = faces
        .iter()
        .max_by(|(n1, _, _), (n2, _, _)| n1.dot(dir).partial_cmp(&n2.dot(dir)).unwrap())
        .unwrap();
    (v0, v1, n)
}

fn clip_segment(a: Vec2, b: Vec2, plane_pt: Vec2, inward_normal: Vec2) -> Option<(Vec2, Vec2)> {
    let da = (a - plane_pt).dot(inward_normal);
    let db = (b - plane_pt).dot(inward_normal);
    if da < 0.0 && db < 0.0 {
        return None;
    }
    if da >= 0.0 && db >= 0.0 {
        return Some((a, b));
    }
    let mid = a + (b - a) * (da / (da - db));
    if da < 0.0 {
        Some((mid, b))
    } else {
        Some((a, mid))
    }
}

fn contact_manifold(
    ref_v0: Vec2,
    ref_v1: Vec2,
    ref_normal: Vec2,
    inc_v0: Vec2,
    inc_v1: Vec2,
) -> Vec<Vec2> {
    let edge = ref_v1 - ref_v0;
    let edge_len = edge.length();
    if edge_len < f32::EPSILON {
        return vec![];
    }
    let tangent = edge / edge_len;
    let seg = match clip_segment(inc_v0, inc_v1, ref_v0, tangent) {
        None => return vec![],
        Some(s) => s,
    };
    let seg = match clip_segment(seg.0, seg.1, ref_v1, -tangent) {
        None => return vec![],
        Some(s) => s,
    };
    [seg.0, seg.1]
        .into_iter()
        .filter(|&v| (v - ref_v0).dot(ref_normal) <= 0.0)
        .collect()
}

fn collide_box_box(a: &Body, b: &Body) -> Option<Collision> {
    let (wa, ha, wb, hb) = match (&a.shape, &b.shape) {
        (Shape::Box { w: wa, h: ha }, Shape::Box { w: wb, h: hb }) => (wa, ha, wb, hb),
        _ => return None,
    };

    let ca = get_corners(a.pos, a.rot, *wa, *ha);
    let cb = get_corners(b.pos, b.rot, *wb, *hb);

    let [xa_a, ya_a] = box_axes(a.rot);
    let [xa_b, ya_b] = box_axes(b.rot);
    let axes = [xa_a, ya_a, xa_b, ya_b];

    let mut min_overlap = f32::INFINITY;
    let mut best_axis = Vec2::ZERO;

    for &axis in &axes {
        match sat_overlap(axis, &ca, &cb) {
            None => return None,
            Some(o) if o < min_overlap => {
                min_overlap = o;
                best_axis = axis;
            }
            _ => {}
        }
    }

    let mut normal = best_axis;
    if normal.dot(a.pos - b.pos) < 0.0 {
        normal = -normal;
    }

    let (ref_v0, ref_v1, ref_face_normal) = best_face(b.pos, b.rot, *wb, *hb, normal);
    let (inc_v0, inc_v1, _) = best_face(a.pos, a.rot, *wa, *ha, -normal);
    let mut contacts = contact_manifold(ref_v0, ref_v1, ref_face_normal, inc_v0, inc_v1);

    if contacts.is_empty() {
        let sup_i = *ca
            .iter()
            .min_by(|p, q| p.dot(normal).partial_cmp(&q.dot(normal)).unwrap())
            .unwrap();
        let sup_j = *cb
            .iter()
            .max_by(|p, q| p.dot(normal).partial_cmp(&q.dot(normal)).unwrap())
            .unwrap();
        contacts = vec![(sup_i + sup_j) * 0.5];
    }

    Some(Collision {
        normal,
        depth: min_overlap,
        contacts,
    })
}

fn collide_ball_ball(a: &Body, b: &Body) -> Option<Collision> {
    let (ra, rb) = match (&a.shape, &b.shape) {
        (Shape::Ball { r: ra }, Shape::Ball { r: rb }) => (ra, rb),
        _ => return None,
    };

    let delta = a.pos - b.pos;
    let dist = delta.length();
    let sum_r = ra + rb;

    if dist >= sum_r {
        return None;
    }

    let normal = if dist < f32::EPSILON {
        Vec2::new(0.0, 1.0)
    } else {
        delta / dist
    };

    Some(Collision {
        normal,
        depth: sum_r - dist,
        contacts: vec![b.pos + normal * *rb],
    })
}

fn collide_ball_box(ball: &Body, b: &Body) -> Option<Collision> {
    let (r, w, h) = match (&ball.shape, &b.shape) {
        (Shape::Ball { r }, Shape::Box { w, h }) => (r, w, h),
        _ => return None,
    };

    let (sin, cos) = b.rot.sin_cos();
    let delta = ball.pos - b.pos;
    let local_x = delta.x * cos + delta.y * sin;
    let local_y = -delta.x * sin + delta.y * cos;
    let hw = w / 2.0;
    let hh = h / 2.0;

    let (normal, depth, contact) = if local_x.abs() < hw && local_y.abs() < hh {
        let dx = hw - local_x.abs();
        let dy = hh - local_y.abs();
        if dx < dy {
            let s = if local_x > 0.0 { 1.0_f32 } else { -1.0 };
            let n = Vec2::new(s * cos, s * sin);
            (n, dx + r, ball.pos - n * *r)
        } else {
            let s = if local_y > 0.0 { 1.0_f32 } else { -1.0 };
            let n = Vec2::new(-s * sin, s * cos);
            (n, dy + r, ball.pos - n * *r)
        }
    } else {
        let cx = local_x.clamp(-hw, hw);
        let cy = local_y.clamp(-hh, hh);
        let closest = b.pos + Vec2::new(cx * cos - cy * sin, cx * sin + cy * cos);
        let diff = ball.pos - closest;
        let dist = diff.length();
        if dist >= *r {
            return None;
        }
        let n = if dist < f32::EPSILON {
            Vec2::new(-cos, -sin)
        } else {
            diff / dist
        };
        (n, r - dist, closest)
    };

    Some(Collision {
        normal,
        depth,
        contacts: vec![contact],
    })
}

fn narrow_phase(a: &Body, b: &Body) -> Option<Collision> {
    match (&a.shape, &b.shape) {
        (Shape::Box { .. }, Shape::Box { .. }) => collide_box_box(a, b),
        (Shape::Ball { .. }, Shape::Ball { .. }) => collide_ball_ball(a, b),
        (Shape::Ball { .. }, Shape::Box { .. }) => collide_ball_box(a, b),
        (Shape::Box { .. }, Shape::Ball { .. }) => collide_ball_box(b, a).map(|c| Collision {
            normal: -c.normal,
            ..c
        }),
    }
}

fn avg_contact(contacts: &[Vec2]) -> Vec2 {
    contacts.iter().fold(Vec2::ZERO, |acc, &p| acc + p) / contacts.len() as f32
}

fn apply_impulse(bodies: &mut Vec<Body>, i: usize, j: usize, col: &Collision) {
    let contact = avg_contact(&col.contacts);
    let normal = col.normal;

    let im_i = if bodies[i].is_static {
        0.0
    } else {
        1.0 / bodies[i].m
    };
    let im_j = if bodies[j].is_static {
        0.0
    } else {
        1.0 / bodies[j].m
    };
    let imoi_i = if bodies[i].is_static {
        0.0
    } else {
        1.0 / bodies[i].moi
    };
    let imoi_j = if bodies[j].is_static {
        0.0
    } else {
        1.0 / bodies[j].moi
    };

    let r_i = contact - bodies[i].pos;
    let r_j = contact - bodies[j].pos;

    let vel_i = if bodies[i].is_static {
        Vec2::ZERO
    } else {
        bodies[i].vel
    };
    let vel_j = if bodies[j].is_static {
        Vec2::ZERO
    } else {
        bodies[j].vel
    };
    let ang_vel_i = if bodies[i].is_static {
        0.0
    } else {
        bodies[i].ang_vel
    };
    let ang_vel_j = if bodies[j].is_static {
        0.0
    } else {
        bodies[j].ang_vel
    };

    let vel_cp_i = vel_i + Vec2::new(-ang_vel_i * r_i.y, ang_vel_i * r_i.x);
    let vel_cp_j = vel_j + Vec2::new(-ang_vel_j * r_j.y, ang_vel_j * r_j.x);
    let v_rel_n = (vel_cp_i - vel_cp_j).dot(normal);

    if v_rel_n >= 0.0 {
        return;
    }

    let r_i_cross_n = r_i.x * normal.y - r_i.y * normal.x;
    let r_j_cross_n = r_j.x * normal.y - r_j.y * normal.x;
    let e = bodies[i].res.min(bodies[j].res);
    let impulse = -(1.0 + e) * v_rel_n
        / (im_i + im_j + r_i_cross_n * r_i_cross_n * imoi_i + r_j_cross_n * r_j_cross_n * imoi_j);

    if !bodies[i].is_static {
        bodies[i].vel += normal * impulse * im_i;
        bodies[i].ang_vel += impulse * r_i_cross_n * imoi_i;
    }
    if !bodies[j].is_static {
        bodies[j].vel -= normal * impulse * im_j;
        bodies[j].ang_vel -= impulse * r_j_cross_n * imoi_j;
    }

    let vel_i2 = if bodies[i].is_static { Vec2::ZERO } else { bodies[i].vel };
    let vel_j2 = if bodies[j].is_static { Vec2::ZERO } else { bodies[j].vel };
    let ang_i2 = if bodies[i].is_static { 0.0 } else { bodies[i].ang_vel };
    let ang_j2 = if bodies[j].is_static { 0.0 } else { bodies[j].ang_vel };

    let vcp_i2 = vel_i2 + Vec2::new(-ang_i2 * r_i.y, ang_i2 * r_i.x);
    let vcp_j2 = vel_j2 + Vec2::new(-ang_j2 * r_j.y, ang_j2 * r_j.x);
    let v_rel2 = vcp_i2 - vcp_j2;

    let tangent = {
        let vt = v_rel2 - normal * v_rel2.dot(normal);
        let len = vt.length();
        if len < f32::EPSILON { return; }
        vt / len
    };

    let v_rel_t = v_rel2.dot(tangent);

    let r_i_cross_t = r_i.x * tangent.y - r_i.y * tangent.x;
    let r_j_cross_t = r_j.x * tangent.y - r_j.y * tangent.x;
    let denom_t = im_i + im_j
        + r_i_cross_t * r_i_cross_t * imoi_i
        + r_j_cross_t * r_j_cross_t * imoi_j;

    if denom_t < f32::EPSILON { return; }

    let jt_raw = -v_rel_t / denom_t;

    let mu = (bodies[i].friction * bodies[j].friction).sqrt();
    let jt = jt_raw.clamp(-mu * impulse.abs(), mu * impulse.abs());

    if !bodies[i].is_static {
        bodies[i].vel += tangent * jt * im_i;
        bodies[i].ang_vel += jt * r_i_cross_t * imoi_i;
    }
    if !bodies[j].is_static {
        bodies[j].vel -= tangent * jt * im_j;
        bodies[j].ang_vel -= jt * r_j_cross_t * imoi_j;
    }
}

fn apply_position_correction(bodies: &mut Vec<Body>, i: usize, j: usize, col: &Collision) {
    const POSITION_BIAS: f32 = 0.4;
    const ALLOWED_PENETRATION: f32 = 0.01;

    let im_i = if bodies[i].is_static {
        0.0
    } else {
        1.0 / bodies[i].m
    };
    let im_j = if bodies[j].is_static {
        0.0
    } else {
        1.0 / bodies[j].m
    };
    let total_inv_mass = im_i + im_j;
    let correction = (col.depth - ALLOWED_PENETRATION).max(0.0) * POSITION_BIAS;

    if total_inv_mass > 0.0 && correction > 0.0 {
        bodies[i].pos += col.normal * correction * (im_i / total_inv_mass);
        bodies[j].pos -= col.normal * correction * (im_j / total_inv_mass);
    }
}

pub fn collision_check(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            if !bodies[i].can_collide || !bodies[j].can_collide {
                continue;
            }
            if bodies[i].is_static && bodies[j].is_static {
                continue;
            }
            if let Some(col) = narrow_phase(&bodies[i], &bodies[j]) {
                apply_impulse(bodies, i, j, &col);
                apply_position_correction(bodies, i, j, &col);
            }
        }
    }
}
