use std::ops::{Add, Mul, MulAssign, Sub};

use crate::{
    camera::Camera,
    color,
    transform::Transform,
    utils::{lerp, map_to_range, plotline, to_argb8},
    Texture,
};
use glam::{Mat4, UVec3, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use shared::{coords_to_index, State, HEIGHT};

use crate::{barycentric_coordinates, edge_function_cw, index_to_coords, WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec4,
    pub color: Vec3,
    pub uv: Vec2,
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self {
            position,
            color,
            uv,
        }
    }
}

impl MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.position *= rhs;
        self.color *= rhs;
        self.uv *= rhs;
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self {
            position,
            color,
            uv,
        }
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self {
            position,
            color,
            uv,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
}

pub enum VerticesOrder {
    ABC,
    ACB,
    BAC,
    BCA,
    CAB,
    CBA,
}

impl Triangle {
    fn new(v0: Vertex, v2: Vertex, v1: Vertex) -> Self {
        Self { v0, v1, v2 }
    }
    pub fn from_vertices(vertices: [&Vertex; 3]) -> Self {
        Triangle {
            v0: *vertices[0],
            v1: *vertices[1],
            v2: *vertices[2],
        }
    }
    pub fn transform(&mut self, matrix: &Mat4) {
        self.v0.position = *matrix * self.v0.position;
        self.v1.position = *matrix * self.v1.position;
        self.v2.position = *matrix * self.v2.position;
    }
    pub fn reorder(&self, order: VerticesOrder) -> Self {
        match order {
            VerticesOrder::ABC => *self,
            VerticesOrder::ACB => Self::new(self.v0, self.v2, self.v1),
            VerticesOrder::BAC => Self::new(self.v1, self.v0, self.v2),
            VerticesOrder::BCA => Self::new(self.v1, self.v2, self.v0),
            VerticesOrder::CAB => Self::new(self.v2, self.v0, self.v1),
            VerticesOrder::CBA => Self::new(self.v2, self.v1, self.v0),
        }
    }
}

pub enum ClipResult {
    None,
    One(Triangle),
    Two((Triangle, Triangle)),
}

pub fn cull_triangle_view_frustum(triangle: &Triangle) -> bool {
    // X-Axis
    if triangle.v0.position.x > triangle.v0.position.w
        && triangle.v1.position.x > triangle.v1.position.w
        && triangle.v2.position.x > triangle.v2.position.w
    {
        return true;
    }
    if triangle.v0.position.x < -triangle.v0.position.w
        && triangle.v1.position.x < -triangle.v1.position.w
        && triangle.v2.position.x < -triangle.v2.position.w
    {
        return true;
    }
    // Y-Axis
    if triangle.v0.position.y > triangle.v0.position.w
        && triangle.v1.position.y > triangle.v1.position.w
        && triangle.v2.position.y > triangle.v2.position.w
    {
        return true;
    }
    if triangle.v0.position.y < -triangle.v0.position.w
        && triangle.v1.position.y < -triangle.v1.position.w
        && triangle.v2.position.y < -triangle.v2.position.w
    {
        return true;
    }
    // Z-Axis
    if triangle.v0.position.z > triangle.v0.position.w
        && triangle.v1.position.z > triangle.v1.position.w
        && triangle.v2.position.z > triangle.v2.position.w
    {
        return true;
    }
    if triangle.v0.position.z < -triangle.v0.position.w
        && triangle.v1.position.z < -triangle.v1.position.w
        && triangle.v2.position.z < -triangle.v2.position.w
    {
        return true;
    }

    if triangle.v0.position.z < 0.0 && triangle.v1.position.z < 0.0 && triangle.v2.position.z < 0.0
    {
        return true;
    }

    false
}

pub fn cull_triangle_backface(triangle: &Triangle) -> bool {
    let normal = (triangle.v1.position.xyz() - triangle.v0.position.xyz())
        .cross(triangle.v2.position.xyz() - triangle.v0.position.xyz());

    normal.dot(-Vec3::Z) >= 0.0
}

enum ClipEvaluation {
    AllPositive,
    OneNegative(u32),
    TwoNegative(u32, u32),
    AllNegative,
}

fn evaluate_clip_vertices(triangle: &Triangle) -> ClipEvaluation {
    let mut negative_count: u32 = 3;
    let mut negative_indices = [true; 3];

    if triangle.v0.position.z > 0.0 {
        negative_indices[0] = false;
        negative_count -= 1;
    }
    if triangle.v1.position.z > 0.0 {
        negative_indices[1] = false;
        negative_count -= 1;
    }
    if triangle.v2.position.z > 0.0 {
        negative_indices[2] = false;
        negative_count -= 1;
    }

    if negative_count == 3 {
        return ClipEvaluation::AllNegative;
    }
    if negative_count == 0 {
        return ClipEvaluation::AllPositive;
    }

    if negative_count == 1 {
        for i in 0..negative_indices.len() {
            if negative_indices[i] == true {
                return ClipEvaluation::OneNegative(i as u32);
            }
        }
    }

    if negative_count == 2 {
        if negative_indices[0] == false {
            return ClipEvaluation::TwoNegative(1, 2);
        }
        if negative_indices[1] == false {
            return ClipEvaluation::TwoNegative(0, 2);
        }
        if negative_indices[2] == false {
            return ClipEvaluation::TwoNegative(0, 1);
        }
    }
    return ClipEvaluation::AllNegative;
}

pub fn clip_triangle_one_negative(triangle: &Triangle) -> (Triangle, Triangle) {
    let alpha_a = (-triangle.v0.position.z) / (triangle.v1.position.z - triangle.v0.position.z);
    let alpha_b = (-triangle.v0.position.z) / (triangle.v2.position.z - triangle.v0.position.z);

    let v0_a = lerp(triangle.v0, triangle.v1, alpha_a);
    let v0_b = lerp(triangle.v0, triangle.v2, alpha_b);

    let mut result_a = *triangle;
    let mut result_b = *triangle;

    result_a.v0 = v0_a;

    result_b.v0 = v0_a;
    result_b.v1 = v0_b;

    return (result_a, result_b);
}

pub fn clip_triangle_two_negatives(triangle: &Triangle) -> Triangle {
    let alpha_a = (-triangle.v0.position.z) / (triangle.v2.position.z - triangle.v0.position.z);
    let alpha_b = (-triangle.v1.position.z) / (triangle.v2.position.z - triangle.v1.position.z);

    let v0 = lerp(triangle.v0, triangle.v2, alpha_a);
    let v1 = lerp(triangle.v1, triangle.v2, alpha_b);

    Triangle {
        v0,
        v1,
        v2: triangle.v2,
    }
}

pub fn clip_cull_triangle(triangle: &Triangle) -> ClipResult {
    // Backface culling
    if cull_triangle_backface(&triangle) {
        return ClipResult::None;
    }
    // Frustum culling
    if cull_triangle_view_frustum(triangle) {
        return ClipResult::None;
    } else {
        let evaluated = evaluate_clip_vertices(triangle);
        // TODO: Clip Triangle
        match evaluated {
            ClipEvaluation::OneNegative(first) => {
                if first == 0
                {
                    return ClipResult::Two(clip_triangle_one_negative(&triangle.reorder(VerticesOrder::ACB)));
                }
                else if first == 1
                {
                    return ClipResult::Two(clip_triangle_one_negative(&triangle.reorder(VerticesOrder::BAC)));
                }
                else if first == 2
                {
                    return ClipResult::Two(clip_triangle_one_negative(&triangle.reorder(VerticesOrder::CBA)));
                }
            }
            ClipEvaluation::TwoNegative(first, second) => {
                if first == 0 && second == 1
                {
                    return ClipResult::One(clip_triangle_two_negatives(&triangle));
                }
                if first == 0 && second == 2
                {
                    return ClipResult::One(clip_triangle_two_negatives(&triangle.reorder(VerticesOrder::ACB)));
                }
                if first == 1 && second == 2
                {
                    return ClipResult::One(clip_triangle_two_negatives(&triangle.reorder(VerticesOrder::BCA)));
                }
            }
            ClipEvaluation::AllPositive => {}
            ClipEvaluation::AllNegative => {}
        }
    }

    // Return original triangle
    return ClipResult::One(*triangle);
}

pub fn draw_triangle(
    vertices: [&Vertex; 3],
    render_state: &RenderState,
    transform: &Transform,
    cam: &Camera,
    viewport: Vec2,
    state: &State,
    zbuff: &mut Vec<f32>,
) {
    let mvp = cam.projection() * cam.view() * transform.local();

    let mut triangle = Triangle::from_vertices(vertices);
    triangle.transform(&mvp);

    let result = clip_cull_triangle(&triangle);

    match result {
        ClipResult::None => {}
        ClipResult::One(tri) => {
            draw_triangle_clipped(&tri, render_state, viewport, state, zbuff);
        }
        ClipResult::Two(tri) => {
            draw_triangle_clipped(&tri.0, render_state, viewport, state, zbuff);
            draw_triangle_clipped(&tri.1, render_state, viewport, state, zbuff);
        }
    }
}

type ShadeFn = fn(&RenderState, [&Vertex; 3], Vec3, f32) -> u32;
pub struct RenderState<'a> {
    texture: Option<&'a Texture>,
    shade_fn: ShadeFn,
}

impl RenderState<'_> {
    pub fn from_shade_fn(shade_fn: ShadeFn, texture: Option<&'_ Texture>) -> RenderState {
        RenderState { texture, shade_fn }
    }
    pub fn draw_texture(texture: Option<&'_ Texture>) -> RenderState {
        RenderState {
            texture: texture,
            shade_fn: draw_texture,
        }
    }
}

pub fn draw_texture(
    state: &RenderState,
    vertices: [&Vertex; 3],
    bary_centric: Vec3,
    correction: f32,
) -> u32 {
    let color: u32;
    let v0 = vertices[0];
    let v1 = vertices[1];
    let v2 = vertices[2];

    match state.texture {
        Some(texture) => {
            let tex_coords =
                bary_centric.x * v0.uv + bary_centric.y * v1.uv + bary_centric.z * v2.uv;
            let tex_coords = tex_coords * correction;
            color = texture.argb_at_uv(tex_coords.x, tex_coords.y);
        }
        None => {
            let vertex_color =
                bary_centric.x * v0.color + bary_centric.y * v1.color + bary_centric.z * v2.color;
            let vertex_color = vertex_color * correction;
            color = to_argb8(
                255,
                (vertex_color.x * 255.0) as u8,
                (vertex_color.y * 255.0) as u8,
                (vertex_color.z * 255.0) as u8,
            );
        }
    }

    color
}

pub fn draw_vertex_color(
    state: &RenderState,
    vertices: [&Vertex; 3],
    bary_centric: Vec3,
    correction: f32,
) -> u32 {
    let color: u32;
    let v0 = vertices[0];
    let v1 = vertices[1];
    let v2 = vertices[2];

    let vertex_color =
        bary_centric.x * v0.color + bary_centric.y * v1.color + bary_centric.z * v2.color;
    let vertex_color = vertex_color * correction;
    color = to_argb8(
        255,
        (vertex_color.x * 255.0) as u8,
        (vertex_color.y * 255.0) as u8,
        (vertex_color.z * 255.0) as u8,
    );

    color
}

pub fn draw_triangle_clipped(
    triangle: &Triangle,
    render_state: &RenderState,
    viewport: Vec2,
    state: &State,
    zbuff: &mut Vec<f32>,
) {
    let rec0 = 1.0 / triangle.v0.position.w;
    let rec1 = 1.0 / triangle.v1.position.w;
    let rec2 = 1.0 / triangle.v2.position.w;

    // This would be the output of the vertex shader (clip space)
    // then we perform perspective division to transform in ndc
    // now x,y,z componend of ndc are between -1 and 1
    let ndc0 = triangle.v0.position * rec0;
    let ndc1 = triangle.v1.position * rec1;
    let ndc2 = triangle.v2.position * rec2;

    let v0 = triangle.v0 * rec0;
    let v1 = triangle.v1 * rec1;
    let v2 = triangle.v2 * rec2;

    // screeen coordinates remapped to window
    let sc0 = glam::vec2(
        map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport.x),
        map_to_range(-ndc0.y, -1.0, 1.0, 0.0, viewport.y),
    );
    let sc1 = glam::vec2(
        map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport.x),
        map_to_range(-ndc1.y, -1.0, 1.0, 0.0, viewport.y),
    );
    let sc2 = glam::vec2(
        map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport.x),
        map_to_range(-ndc2.y, -1.0, 1.0, 0.0, viewport.y),
    );

    let mut bounds = BoundingBox2D::get_bounds_from_triangle(&[sc0, sc1, sc2]);
    bounds.clamp(Vec2::ZERO, viewport);

    // Loop over positions instead of pixels, to only update the part of the screen that is needed.
    for y in bounds.min.y as usize..bounds.max.y as usize {
        for x in bounds.min.x as usize..bounds.max.x as usize {
            let coords = Vec2::new(x as f32, y as f32) + 0.5;
            let pixel_id = coords_to_index(coords);

            // Ensure pixel is within bounds
            if pixel_id >= WIDTH * HEIGHT {
                continue;
            }

            let area = edge_function_cw(sc0, sc1, sc2);
            let bary = barycentric_coordinates(coords, sc0, sc1, sc2, area);
            if let Some(b) = bary {
                let correction = b.x * rec0 + b.y * rec1 + b.z * rec2;
                let depth = correction;
                let correction = 1.0 / correction;

                if depth <= zbuff[pixel_id] {
                    zbuff[pixel_id] = depth;
                    let color =
                        (render_state.shade_fn)(render_state, [&v0, &v1, &v2], b, correction);
                    state.draw(x as u16, y as u16, color);
                }
            }
        }
    }
}

pub struct Mesh {
    triangles: Vec<UVec3>,
    vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            vertices: Vec::new(),
        }
    }

    pub fn triangles(&self) -> &Vec<UVec3> {
        &self.triangles
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn get_triangle_vertices(&self, triangle: UVec3) -> [&Vertex; 3] {
        [
            &self.vertices[triangle.x as usize],
            &self.vertices[triangle.y as usize],
            &self.vertices[triangle.z as usize],
        ]
    }

    pub fn add_vertices(&mut self, triangles: &mut Vec<UVec3>, vertices: &mut Vec<Vertex>) {
        self.triangles.append(triangles);
        self.vertices.append(vertices);
    }

    pub fn draw_mesh(
        &self,
        render_state: &RenderState,
        transform: &Transform,
        cam: &Camera,
        viewport: Vec2,
        state: &State,
        zbuff: &mut Vec<f32>,
    ) {
        for triangle in &self.triangles {
            let vertices = self.get_triangle_vertices(*triangle);
            draw_triangle(
                vertices,
                render_state,
                transform,
                cam,
                viewport,
                state,
                zbuff,
            );
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BoundingBox2D {
    pub min: Vec2,
    pub max: Vec2,
}

impl BoundingBox2D {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn get_bounds_from_triangle(positions: &[Vec2; 3]) -> Self {
        let min = Vec2::new(
            positions[0].x.min(positions[1].x.min(positions[2].x)),
            positions[0].y.min(positions[1].y.min(positions[2].y)),
        );
        let max = Vec2::new(
            positions[0].x.max(positions[1].x.max(positions[2].x)),
            positions[0].y.max(positions[1].y.max(positions[2].y)),
        );

        Self { min, max }
    }

    pub fn clamp(&mut self, min: Vec2, max: Vec2) {
        if self.min.x < min.x {
            self.min.x = min.x;
        }
        if self.min.y < min.y {
            self.min.y = min.y;
        }
        if self.max.x > max.x {
            self.max.x = max.x;
        }
        if self.max.y > max.y {
            self.max.y = max.y;
        }
    }
}
