use crate::Vec2;
use crate::bodies::PhysicsBox;
fn get_corners(body: &PhysicsBox) -> Vec<Vec2> {
    let cos = body.rot.cos();
    let sin = body.rot.sin();
    let hw = body.w / 2.0;
    let hh = body.h / 2.0;
    vec![
        Vec2::new(-hw * cos - (-hh * sin), -hw * sin + (-hh * cos)),
        Vec2::new(-hw * cos - (hh * sin), -hw * sin + (hh * cos)),
        Vec2::new(hw * cos - (-hh * sin), hw * sin + (-hh * cos)),
        Vec2::new(hw * cos - (hh * sin), hw * sin + (hh * cos)),
    ]
}
// Just boxes for now (because rust is hard)
pub fn collision_check(bodies: &mut Vec<PhysicsBox>) {
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            if !bodies[i].can_collide || !bodies[j].can_collide {
                continue;
            }
            // Box i and j vertices
            let i_corners: Vec<Vec2> = get_corners(&bodies[i])
                .iter()
                .map(|c| *c + bodies[i].pos)
                .collect();
            let j_corners: Vec<Vec2> = get_corners(&bodies[j])
                .iter()
                .map(|c| *c + bodies[j].pos)
                .collect();

            fn get_dot_products(
                axis: Vec2,
                i_corners: &Vec<Vec2>,
                j_corners: &Vec<Vec2>,
            ) -> Option<f32> {
                let i_dots: Vec<f32> = i_corners.iter().map(|&x| x.dot(axis)).collect();
                let j_dots: Vec<f32> = j_corners.iter().map(|&x| x.dot(axis)).collect();

                let i_min_max: (f32, f32) = i_dots.iter().fold(
                    (f32::INFINITY, f32::NEG_INFINITY),
                    |(min_so_far, max_so_far), &current| {
                        (f32::min(min_so_far, current), f32::max(max_so_far, current))
                    },
                );
                let j_min_max: (f32, f32) = j_dots.iter().fold(
                    (f32::INFINITY, f32::NEG_INFINITY),
                    |(min_so_far, max_so_far), &current| {
                        (f32::min(min_so_far, current), f32::max(max_so_far, current))
                    },
                );

                let overlap =
                    f32::min(i_min_max.1, j_min_max.1) - f32::max(i_min_max.0, j_min_max.0);

                if overlap < 0.0 {
                    return None;
                } else {
                    return Some(overlap);
                }
            }

            let i_box_x_axis = Vec2::new(bodies[i].rot.cos(), bodies[i].rot.sin());
            let i_box_y_axis = Vec2::new(-bodies[i].rot.sin(), bodies[i].rot.cos());

            let j_box_x_axis = Vec2::new(bodies[j].rot.cos(), bodies[j].rot.sin());
            let j_box_y_axis = Vec2::new(-bodies[j].rot.sin(), bodies[j].rot.cos());

            let axes = [i_box_x_axis, i_box_y_axis, j_box_x_axis, j_box_y_axis];

            let axes_overlaps = axes
                .iter()
                .map(|&axis| {
                    get_dot_products(axis, &i_corners, &j_corners).map(|overlap| (axis, overlap))
                })
                .collect::<Option<Vec<_>>>();
            let overlaps = match axes_overlaps {
                None => continue,
                Some(v) => v,
            };
            let min_overlap = overlaps
                .iter()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            let (mut normal, depth) = *min_overlap;

            if (bodies[i].pos - bodies[j].pos).dot(normal) < 0.0 {
                normal = -normal;
            }
            // move bodies apart relative to mass (just so they aren't overlapping)
            let m_i = bodies[i].m;
            let m_j = bodies[j].m;
            bodies[i].pos += normal * depth * (m_j / (m_i + m_j));
            bodies[j].pos -= normal * depth * (m_i / (m_j + m_i));

            // normal points from j to i
            let support_i = *i_corners
                .iter()
                .min_by(|a, b| a.dot(normal).partial_cmp(&b.dot(normal)).unwrap())
                .unwrap(); // deepest point of i along -normal (i.e. min projection on normal)
            let support_j = *j_corners
                .iter()
                .max_by(|a, b| a.dot(normal).partial_cmp(&b.dot(normal)).unwrap())
                .unwrap(); // deepest point of j along +normal

            let contact_point = (support_i + support_j) * 0.5;
            let r_i = contact_point - bodies[i].pos;
            let r_j = contact_point - bodies[j].pos;
            let vel_i = bodies[i].vel;
            let vel_j = bodies[j].vel;
            let ang_vel_i = bodies[i].ang_vel;
            let ang_vel_j = bodies[j].ang_vel;
            let moi_i = bodies[i].moi;
            let moi_j = bodies[j].moi;
            let res_i = bodies[i].res;
            let res_j = bodies[j].res;

            let ang_vel_i_cross_r_i: Vec2 = Vec2::new(-ang_vel_i * r_i.y, ang_vel_i * r_i.x);
            let ang_vel_j_cross_r_j: Vec2 = Vec2::new(-ang_vel_j * r_j.y, ang_vel_j * r_j.x);
            let v_rel = (vel_i + ang_vel_i_cross_r_i) - (vel_j + ang_vel_j_cross_r_j);
            let v_rel_n = v_rel.dot(normal);

            // Skip applying force to them if they're already moving apart
            if v_rel_n > 0.0 {
                continue;
            }
            let r_i_cross_n = r_i.x * normal.y - r_i.y * normal.x;
            let r_j_cross_n = r_j.x * normal.y - r_j.y * normal.x;

            let e = (res_i + res_j) / 2.0;
            let impulse = -(1.0 + e) * v_rel_n
                / (1.0 / m_i
                    + 1.0 / m_j
                    + r_i_cross_n * r_i_cross_n / moi_i
                    + r_j_cross_n * r_j_cross_n / moi_j);
            // Apply forces
            bodies[i].vel += normal * impulse / m_i;
            bodies[j].vel -= normal * impulse / m_j;

            bodies[i].ang_vel += impulse * r_i_cross_n / moi_i;
            bodies[j].ang_vel -= impulse * r_j_cross_n / moi_j;
        }
    }
}
