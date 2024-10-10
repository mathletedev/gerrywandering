use crate::{
    boid::{Boid, Party},
    settings::{Settings, NUM_PARTIES, WINDOW_WIDTH},
};

#[derive(Clone, Copy, Default)]
pub struct Bounds {
    pub left: f32,
    pub bottom: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct Node {
    pub party_count: [u32; NUM_PARTIES],
    pub bounds: Bounds,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

type MutNodeRef<'a> = &'a mut Option<Box<Node>>;

pub fn favours(party_count: [u32; NUM_PARTIES]) -> Option<Party> {
    match party_count[0].cmp(&party_count[1]) {
        std::cmp::Ordering::Greater => Some(Party::RED),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Less => Some(Party::BLUE),
    }
}

pub fn gerrymander(node: MutNodeRef, favour: Party) {
    let node = match node {
        Some(node) => node,
        None => return,
    };

    gerrymander(&mut node.left, favour);
    gerrymander(&mut node.right, favour);

    let left = match &node.left {
        Some(node) => node,
        None => return,
    };

    let right = match &node.right {
        Some(node) => node,
        None => return,
    };

    if favours(left.party_count) != Some(favour) && favours(right.party_count) != Some(favour) {
        node.left = None;
        node.right = None;
    }
}

pub fn count_parties(node: MutNodeRef, bounds: Bounds, boids: &[Boid], settings: &Settings) {
    let node = match node {
        Some(node) => node,
        None => return,
    };

    node.bounds = bounds;

    if bounds.width.min(bounds.height) > WINDOW_WIDTH as f32 * settings.district_min_size {
        node.left = Some(Box::default());
        node.right = Some(Box::default());

        let mut left = bounds.left;
        let mut bottom = bounds.bottom;
        let mut width = bounds.width;
        let mut height = bounds.height;

        // cut longest direction
        if width > height {
            width /= 2.0;
            left += width;
        } else {
            height /= 2.0;
            bottom += height;
        }

        count_parties(
            &mut node.left,
            Bounds {
                left: bounds.left,
                bottom: bounds.bottom,
                width,
                height,
            },
            boids,
            settings,
        );
        count_parties(
            &mut node.right,
            Bounds {
                left,
                bottom,
                width,
                height,
            },
            boids,
            settings,
        );

        if let Some(left) = &node.left {
            (0..NUM_PARTIES).for_each(|i| {
                node.party_count[i] += left.party_count[i];
            });
        }
        if let Some(right) = &node.right {
            (0..NUM_PARTIES).for_each(|i| {
                node.party_count[i] += right.party_count[i];
            });
        }

        return;
    }

    let mut party_count = [0; 2];

    boids.iter().for_each(|boid| {
        if boid.position.x < bounds.left
            || boid.position.x >= bounds.left + bounds.width
            || boid.position.y < bounds.bottom
            || boid.position.y >= bounds.bottom + bounds.height
        {
            return;
        }

        match &boid.party {
            Some(party) => match party {
                Party::RED => party_count[0] += 1,
                Party::BLUE => party_count[1] += 1,
            },
            None => {}
        }
    });

    node.party_count = party_count;
}

pub fn count_districts(node: &Option<Box<Node>>) -> [u32; NUM_PARTIES] {
    let mut res = [0; NUM_PARTIES];

    let node = match node {
        Some(node) => node,
        None => return res,
    };

    if node.left.is_none() && node.right.is_none() {
        match favours(node.party_count) {
            Some(Party::RED) => return [1, 0],
            Some(Party::BLUE) => return [0, 1],
            None => return [0, 0],
        };
    }

    let left = count_districts(&node.left);
    let right = count_districts(&node.right);

    (0..NUM_PARTIES).for_each(|i| {
        res[i] += left[i] + right[i];
    });

    res
}
