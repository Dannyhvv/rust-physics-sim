use crate::Vec2;
use crate::bodies::PhysicsBox;

fn get_corners(body: &PhysicsBox) -> [Vec2; 4] {
    let cos = body.rot.cos();
    let sin = body.rot.sin();
    let hw = body.w / 2.0;
    let hh = body.h / 2.0;
    let p = body.pos;
    [
        p + Vec2::new(-hw * cos + hh * sin, -hw * sin - hh * cos), // (-hw, -hh)
        p + Vec2::new(hw * cos + hh * sin, hw * sin - hh * cos),   // ( hw, -hh)
        p + Vec2::new(hw * cos - hh * sin, hw * sin + hh * cos),   // ( hw,  hh)
        p + Vec2::new(-hw * cos - hh * sin, -hw * sin + hh * cos), // (-hw,  hh)
    ]
}

fn sat_overlap(axis: Vec2, ca: &[Vec2; 4], cb: &[Vec2; 4]) -> Option<f32> {
    let project = |corners: &[Vec2; 4]| -> (f32, f32) {
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

fn best_face(body: &PhysicsBox, dir: Vec2) -> (Vec2, Vec2) {
    let xa = Vec2::new(body.rot.cos(), body.rot.sin());
    let ya = Vec2::new(-body.rot.sin(), body.rot.cos());
    let hw = body.w / 2.0;
    let hh = body.h / 2.0;
    let p = body.pos;

    let faces = [
        (xa, p + xa * hw - ya * hh, p + xa * hw + ya * hh),
        (-xa, p - xa * hw + ya * hh, p - xa * hw - ya * hh),
        (ya, p - xa * hw + ya * hh, p + xa * hw + ya * hh),
        (-ya, p + xa * hw - ya * hh, p - xa * hw - ya * hh),
    ];

    let (_, v0, v1) = *faces
        .iter()
        .max_by(|(n1, _, _), (n2, _, _)| n1.dot(dir).partial_cmp(&n2.dot(dir)).unwrap())
        .unwrap();
    (v0, v1)
}

fn clip_segment(a: Vec2, b: Vec2, plane_pt: Vec2, inward_normal: Vec2) -> Option<(Vec2, Vec2)> {
    let da = (a - plane_pt).dot(inward_normal);
    let db = (b - plane_pt).dot(inward_normal);
    if da < 0.0 && db < 0.0 {
        return None; // Both outside
    }
    if da >= 0.0 && db >= 0.0 {
        return Some((a, b));
    }
    let t = da / (da - db);
    let mid = a + (b - a) * t;
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

    let mut contacts = Vec::new();
    for &v in &[seg.0, seg.1] {
        if (v - ref_v0).dot(ref_normal) <= 0.0 {
            contacts.push(v);
        }
    }
    contacts
}

// Just boxes for now (because rust is hard)
pub fn collision_check(bodies: &mut Vec<PhysicsBox>) {
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            if !bodies[i].can_collide || !bodies[j].can_collide {
                continue;
            }
            if bodies[i].is_static && bodies[j].is_static {
                continue;
            }

            let ca = get_corners(&bodies[i]);
            let cb = get_corners(&bodies[j]);

            let xa_i = Vec2::new(bodies[i].rot.cos(), bodies[i].rot.sin());
            let ya_i = Vec2::new(-bodies[i].rot.sin(), bodies[i].rot.cos());
            let xa_j = Vec2::new(bodies[j].rot.cos(), bodies[j].rot.sin());
            let ya_j = Vec2::new(-bodies[j].rot.sin(), bodies[j].rot.cos());

            let axes = [xa_i, ya_i, xa_j, ya_j];

            let mut found_separation = false;
            let mut min_overlap = f32::INFINITY;
            let mut best_axis = Vec2::ZERO;

            for &axis in &axes {
                match sat_overlap(axis, &ca, &cb) {
                    None => {
                        found_separation = true;
                        break;
                    }
                    Some(o) => {
                        if o < min_overlap {
                            min_overlap = o;
                            best_axis = axis;
                        }
                    }
                }
            }

            if found_separation {
                continue;
            }

            let depth = min_overlap;
            let mut normal = best_axis;

            let dir = bodies[i].pos - bodies[j].pos;
            if normal.dot(dir) < 0.0 {
                normal = -normal;
            }

            let (ref_v0, ref_v1) = best_face(&bodies[j], normal);
            let (inc_v0, inc_v1) = best_face(&bodies[i], -normal);

            let contacts = contact_manifold(ref_v0, ref_v1, normal, inc_v0, inc_v1);

            let contact_point = if !contacts.is_empty() {
                contacts.iter().fold(Vec2::ZERO, |acc, &p| acc + p) / contacts.len() as f32
            } else {
                let sup_i = *ca
                    .iter()
                    .min_by(|a, b| a.dot(normal).partial_cmp(&b.dot(normal)).unwrap())
                    .unwrap();
                let sup_j = *cb
                    .iter()
                    .max_by(|a, b| a.dot(normal).partial_cmp(&b.dot(normal)).unwrap())
                    .unwrap();
                (sup_i + sup_j) * 0.5
            };

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

            let r_i = contact_point - bodies[i].pos;
            let r_j = contact_point - bodies[j].pos;

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
            let v_rel = vel_cp_i - vel_cp_j;
            let v_rel_n = v_rel.dot(normal);

            if v_rel_n < 0.0 {
                let r_i_cross_n = r_i.x * normal.y - r_i.y * normal.x;
                let r_j_cross_n = r_j.x * normal.y - r_j.y * normal.x;

                let e = (bodies[i].res + bodies[j].res) / 2.0;

                let impulse = -(1.0 + e) * v_rel_n
                    / (im_i
                        + im_j
                        + r_i_cross_n * r_i_cross_n * imoi_i
                        + r_j_cross_n * r_j_cross_n * imoi_j);

                if !bodies[i].is_static {
                    bodies[i].vel += normal * impulse * im_i;
                    bodies[i].ang_vel += impulse * r_i_cross_n * imoi_i;
                }
                if !bodies[j].is_static {
                    bodies[j].vel -= normal * impulse * im_j;
                    bodies[j].ang_vel -= impulse * r_j_cross_n * imoi_j;
                }
            }

            const POSITION_BIAS: f32 = 0.4;
            const ALLOWED_PENETRATION: f32 = 0.01;

            let correction = (depth - ALLOWED_PENETRATION).max(0.0) * POSITION_BIAS;
            let total_inv_mass = im_i + im_j;

            if total_inv_mass > 0.0 && correction > 0.0 {
                bodies[i].pos += normal * correction * (im_i / total_inv_mass);
                bodies[j].pos -= normal * correction * (im_j / total_inv_mass);
            }
        }
    }
}
