use crate::utils::ply::{read_aligned_points, VertAligned};
use crate::utils::Locatedf32;
use std::path::Path;

#[derive(Clone, Copy, Debug, Default)]
/// Points to a node in an Octree
struct OctreeNodePointer(usize);
#[derive(Clone, Copy, Debug, Default)]
/// Points to a vertex in an Octree
struct OctreeVertexPointer(usize);

fn half(min_bounds: &glm::Vec3, max_bounds: &glm::Vec3) -> glm::Vec3 {
    return (max_bounds - min_bounds) * 0.5 + min_bounds;
}

struct Octree<T: Locatedf32, const LEAF_CAPACITY: usize, const MAX_DEPTH: usize> {
    nodes: Vec<OctreeNode>,
    vertices: Vec<T>,
    min_bounds: glm::Vec3,
    max_bounds: glm::Vec3,
    size: glm::Vec3,
}

#[derive(Debug)]
enum OctreeNode {
    Leaf { contents: Vec<OctreeVertexPointer> },
    Branch { nodes: [OctreeNodePointer; 8] },
}

impl OctreeNode {
    pub fn new() -> Self {
        OctreeNode::Leaf {
            contents: Vec::new(),
        }
    }

    pub fn new_branch<T: Locatedf32>(
        nodes: &mut Vec<OctreeNode>,
        vertices: &Vec<T>,
        contents: &Vec<OctreeVertexPointer>,
        min_bounds: &glm::Vec3,
        max_bounds: &glm::Vec3,
    ) -> Self {
        let mut nodesn = [OctreeNodePointer(0); 8];
        for i in 0..8 {
            nodesn[i] = OctreeNodePointer(nodes.len());
            nodes.push(OctreeNode::new());
        }
        let mut distribution = [0; 8];
        for vp in contents {
            let v = &vertices[vp.0];
            let h = half(min_bounds, max_bounds);
            let mut index = 0;
            if v.x() > h.x {
                index += 4;
            }
            if v.y() > h.y {
                index += 2;
            }
            if v.z() > h.z {
                index += 1;
            }
            distribution[index] += 1;
            if let OctreeNode::Leaf { contents } = &mut nodes[nodesn[index].0] {
                contents.push(*vp);
            }
        }
        //for d in distribution.iter() {
        //    if *d == contents.len() {
        //        panic!("Degenerate octree!");
        //    }
        //}

        OctreeNode::Branch { nodes: nodesn }
    }
}

impl<T: Locatedf32, const LEAF_CAPACITY: usize, const MAX_DEPTH: usize>
    Octree<T, LEAF_CAPACITY, MAX_DEPTH>
{
    pub fn new(min_bounds: glm::Vec3, max_bounds: glm::Vec3, vertices: Vec<T>) -> Self {
        let size = max_bounds - min_bounds;

        if size.x <= 0. || size.y <= 0. || size.z <= 0. {
            panic!("Minimum has to be smaller than maximum for octree initializer!");
        }

        let mut nodes = vec![OctreeNode::new()];

        println!("Adding {} vertices!", vertices.len());
        let mut i = 0;
        for v in &vertices {
            if v.x() <= max_bounds.x
                && v.x() >= min_bounds.x
                && v.y() <= max_bounds.y
                && v.y() >= min_bounds.y
                && v.z() <= max_bounds.z
                && v.z() >= min_bounds.z
            {
                Self::add_vertex(
                    &mut nodes,
                    &vertices,
                    OctreeVertexPointer(i),
                    &min_bounds,
                    &max_bounds,
                );
            }
            i += 1;
            if i == 10000 {
                break;
            }
        }

        i = 0;
        for node in &nodes {
            println!("Node {}: {:?}", i, node);
            i += 1;
        }

        println!("Returning octree!");

        Octree {
            nodes,
            vertices,
            min_bounds,
            max_bounds,
            size,
        }
    }

    fn add_vertex(
        nodes: &mut Vec<OctreeNode>,
        vertices: &Vec<T>,
        added_vertex: OctreeVertexPointer,
        min_bounds: &glm::Vec3,
        max_bounds: &glm::Vec3,
    ) {
        let mut min_bounds = min_bounds.clone();
        let mut max_bounds = max_bounds.clone();
        let mut pointer = OctreeNodePointer(0);

        let v = &vertices[added_vertex.0];
        let mut depth = 0;
        loop {
            depth += 1;
            match &mut nodes[pointer.0] {
                OctreeNode::Branch { nodes } => {
                    let mut index = 0;
                    let h = half(&min_bounds, &max_bounds);
                    let h2 = (max_bounds - min_bounds) * 0.5;

                    if v.x() > h.x {
                        index += 4;
                        min_bounds.x += h2.x;
                    } else {
                        max_bounds.x -= h2.x;
                    }
                    if v.y() > h.y {
                        index += 2;
                        min_bounds.y += h2.y;
                    } else {
                        max_bounds.y -= h2.y;
                    }
                    if v.z() > h.z {
                        index += 1;
                        min_bounds.z += h2.z;
                    } else {
                        max_bounds.z -= h2.z;
                    }
                    pointer = nodes[index];
                }
                OctreeNode::Leaf { contents: _ } => {
                    break;
                }
            }
        }

        let mut split = false;

        if let OctreeNode::Leaf { contents } = &mut nodes[pointer.0] {
            if contents.len() > LEAF_CAPACITY && depth < MAX_DEPTH {
                split = true;
            } else {
                contents.push(added_vertex);
            }
        } else {
            panic!("Branch node in Octree has no children!");
        }

        if split {
            let contents = match &nodes[pointer.0] {
                OctreeNode::Leaf { contents } => contents.clone(),
                OctreeNode::Branch { nodes: _ } => {
                    panic!("unreachable!");
                }
            };
            let new_node =
                OctreeNode::new_branch(nodes, vertices, &contents, &min_bounds, &max_bounds);
            nodes[pointer.0] = new_node;
        }
    }
}

fn create_voxels<P: AsRef<Path>>(file: P) -> std::io::Result<Octree<VertAligned, 20, 9>> {
    let points = read_aligned_points(file)?;
    println!("Available points: {}", points.len());

    let octree = Octree::new(glm::vec3(-5., -5., -5.), glm::vec3(5., 5., 5.), points);

    Ok(octree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn basic_test() {
        let _oct = create_voxels("assets/graphics/ply/test/minimal.points").unwrap();
    }
}
