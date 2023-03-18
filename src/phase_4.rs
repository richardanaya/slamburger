use crate::common::{Descriptor, KeyPoint};

pub fn match_features(
    keypoints1: &[KeyPoint],
    descriptors1: &[Descriptor],
    keypoints2: &[KeyPoint],
    descriptors2: &[Descriptor],
    max_hamming_distance: usize,
) -> Vec<(KeyPoint, KeyPoint)> {
    let mut matches = Vec::new();

    for (keypoint1, descriptor1) in keypoints1.iter().zip(descriptors1) {
        let mut best_distance = max_hamming_distance;
        let mut best_match = None;

        for (keypoint2, descriptor2) in keypoints2.iter().zip(descriptors2) {
            let distance = hamming_distance(&descriptor1.0, &descriptor2.0);

            if distance < best_distance {
                best_distance = distance;
                best_match = Some(keypoint2);
            }
        }

        if let Some(matched_keypoint) = best_match {
            matches.push((*keypoint1, *matched_keypoint));
        }
    }

    matches
}

fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> usize {
    let a: u32 = bytes1
        .iter()
        .zip(bytes2)
        .map(|(byte1, byte2)| (byte1 ^ byte2).count_ones())
        .sum();
    a as usize
}
