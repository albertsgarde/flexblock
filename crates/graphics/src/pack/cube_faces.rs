use utils::Vertex3D;

/// TODO: FIX ORIENTATION OF CUBE FACE
#[allow(clippy::too_many_arguments)]
pub fn left(
    x: f32,
    y0: f32,
    z0: f32,
    y1: f32,
    z1: f32,
    r: f32,
    g: f32,
    b: f32,
    start_index: u32,
) -> (Vec<Vertex3D>, Vec<u32>) {
    let vertices = vec![
        Vertex3D {
            x,
            y: y0,
            z: z0,
            r,
            g,
            b,
            u: 0.0,
            v: 0.0,
        },
        Vertex3D {
            x,
            y: y1,
            z: z0,
            r,
            g,
            b,
            u: 1.0,
            v: 0.0,
        },
        Vertex3D {
            x,
            y: y0,
            z: z1,
            r,
            g,
            b,
            u: 0.0,
            v: 1.0,
        },
        Vertex3D {
            x,
            y: y1,
            z: z1,
            r,
            g,
            b,
            u: 1.0,
            v: 1.0,
        },
    ];

    let elements = vec![
        start_index + 2,
        start_index + 1,
        start_index,
        start_index + 2,
        start_index + 3,
        start_index + 1,
    ];

    (vertices, elements)
}
/// TODO: FIX ORIENTATION OF CUBE FACE
#[allow(clippy::too_many_arguments)]
pub fn right(
    x: f32,
    y0: f32,
    z0: f32,
    y1: f32,
    z1: f32,
    r: f32,
    g: f32,
    b: f32,
    start_index: u32,
) -> (Vec<Vertex3D>, Vec<u32>) {
    let vertices = vec![
        Vertex3D {
            x,
            y: y0,
            z: z0,
            r,
            g,
            b,
            u: 0.0,
            v: 0.0,
        },
        Vertex3D {
            x,
            y: y1,
            z: z0,
            r,
            g,
            b,
            u: 1.0,
            v: 0.0,
        },
        Vertex3D {
            x,
            y: y0,
            z: z1,
            r,
            g,
            b,
            u: 0.0,
            v: 1.0,
        },
        Vertex3D {
            x,
            y: y1,
            z: z1,
            r,
            g,
            b,
            u: 1.0,
            v: 1.0,
        },
    ];
    let elements = vec![
        start_index,
        start_index + 1,
        start_index + 2,
        start_index + 1,
        start_index + 3,
        start_index + 2,
    ];

    (vertices, elements)
}

/// TODO: FIX ORIENTATION OF CUBE FACE
#[allow(clippy::too_many_arguments)]
pub fn top(
    y: f32,
    x0: f32,
    z0: f32,
    x1: f32,
    z1: f32,
    r: f32,
    g: f32,
    b: f32,
    start_index: u32,
) -> (Vec<Vertex3D>, Vec<u32>) {
    let vertices = vec![
        Vertex3D {
            x: x0,
            y,
            z: z0,
            r,
            g,
            b,
            u: 0.0,
            v: 0.0,
        },
        Vertex3D {
            x: x1,
            y,
            z: z0,
            r,
            g,
            b,
            u: 1.0,
            v: 0.0,
        },
        Vertex3D {
            x: x0,
            y,
            z: z1,
            r,
            g,
            b,
            u: 0.0,
            v: 1.0,
        },
        Vertex3D {
            x: x1,
            y,
            z: z1,
            r,
            g,
            b,
            u: 1.0,
            v: 1.0,
        },
    ];

    let elements = vec![
        start_index + 2,
        start_index + 1,
        start_index,
        start_index + 2,
        start_index + 3,
        start_index + 1,
    ];

    (vertices, elements)
}

/// TODO: FIX ORIENTATION OF CUBE FACE
#[allow(clippy::too_many_arguments)]
pub fn bottom(
    y: f32,
    x0: f32,
    z0: f32,
    x1: f32,
    z1: f32,
    r: f32,
    g: f32,
    b: f32,
    start_index: u32,
) -> (Vec<Vertex3D>, Vec<u32>) {
    let vertices = vec![
        Vertex3D {
            x: x0,
            y,
            z: z0,
            r,
            g,
            b,
            u: 0.0,
            v: 0.0,
        },
        Vertex3D {
            x: x1,
            y,
            z: z0,
            r,
            g,
            b,
            u: 1.0,
            v: 0.0,
        },
        Vertex3D {
            x: x0,
            y,
            z: z1,
            r,
            g,
            b,
            u: 0.0,
            v: 1.0,
        },
        Vertex3D {
            x: x1,
            y,
            z: z1,
            r,
            g,
            b,
            u: 1.0,
            v: 1.0,
        },
    ];

    let elements = vec![
        start_index,
        start_index + 1,
        start_index + 2,
        start_index + 1,
        start_index + 3,
        start_index + 2,
    ];

    (vertices, elements)
}

/// TODO: FIX ORIENTATION OF CUBE FACE
#[allow(clippy::too_many_arguments)]
pub fn front(
    z: f32,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    r: f32,
    g: f32,
    b: f32,
    start_index: u32,
) -> (Vec<Vertex3D>, Vec<u32>) {
    let vertices = vec![
        Vertex3D {
            x: x0,
            y: y0,
            z,
            r,
            g,
            b,
            u: 0.0,
            v: 0.0,
        },
        Vertex3D {
            x: x1,
            y: y0,
            z,
            r,
            g,
            b,
            u: 1.0,
            v: 0.0,
        },
        Vertex3D {
            x: x0,
            y: y1,
            z,
            r,
            g,
            b,
            u: 0.0,
            v: 1.0,
        },
        Vertex3D {
            x: x1,
            y: y1,
            z,
            r,
            g,
            b,
            u: 1.0,
            v: 1.0,
        },
    ];

    let elements = vec![
        start_index,
        start_index + 1,
        start_index + 2,
        start_index + 1,
        start_index + 3,
        start_index + 2,
    ];

    (vertices, elements)
}

/// TODO: FIX ORIENTATION OF CUBE FACE
#[allow(clippy::too_many_arguments)]
pub fn back(
    z: f32,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    r: f32,
    g: f32,
    b: f32,
    start_index: u32,
) -> (Vec<Vertex3D>, Vec<u32>) {
    let vertices = vec![
        Vertex3D {
            x: x0,
            y: y0,
            z,
            r,
            g,
            b,
            u: 0.0,
            v: 0.0,
        },
        Vertex3D {
            x: x1,
            y: y0,
            z,
            r,
            g,
            b,
            u: 1.0,
            v: 0.0,
        },
        Vertex3D {
            x: x0,
            y: y1,
            z,
            r,
            g,
            b,
            u: 0.0,
            v: 1.0,
        },
        Vertex3D {
            x: x1,
            y: y1,
            z,
            r,
            g,
            b,
            u: 1.0,
            v: 1.0,
        },
    ];

    let elements = vec![
        start_index + 2,
        start_index + 1,
        start_index,
        start_index + 2,
        start_index + 3,
        start_index + 1,
    ];

    (vertices, elements)
}
