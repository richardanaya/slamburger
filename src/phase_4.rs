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

/****************/
/*  UNIT TESTS  */
/****************/

#[cfg(test)]
mod tests {
    use crate::common::{Descriptor, KeyPoint};

    #[test]
    fn test_match_features() {
        let keypoints1 = [
            KeyPoint {
                x: 0.0,
                y: 0.0,
                orientation: 0.0,
            },
            KeyPoint {
                x: 1.0,
                y: 1.0,
                orientation: 0.0,
            },
            KeyPoint {
                x: 2.0,
                y: 2.0,
                orientation: 0.0,
            },
        ];
        let descriptors1 = [
            Descriptor(vec![0b00000000, 0b00000000, 0b00000000, 0b00000000]),
            Descriptor(vec![0b00000000, 0b00000000, 0b00000000, 0b00000011]),
            Descriptor(vec![0b01101000, 0b01000000, 0b00010000, 0b00000011]),
        ];
        let keypoints2 = [
            KeyPoint {
                x: 0.0,
                y: 0.0,
                orientation: 0.0,
            },
            KeyPoint {
                x: 1.0,
                y: 1.0,
                orientation: 0.0,
            },
            KeyPoint {
                x: 2.0,
                y: 2.0,
                orientation: 0.0,
            },
        ];
        let descriptors2 = [
            Descriptor(vec![0b00000000, 0b00000000, 0b00000000, 0b00000000]),
            Descriptor(vec![0b00000000, 0b00000000, 0b00000000, 0b00000011]),
            // odd ball
            Descriptor(vec![0b01101000, 0b01000000, 0b00010000, 0b00000011]),
        ];
        let matches =
            super::match_features(&keypoints1, &descriptors1, &keypoints2, &descriptors2, 0);
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].0, keypoints1[0]);
        assert_eq!(matches[0].1, keypoints2[0]);
        assert_eq!(matches[1].0, keypoints1[1]);
        assert_eq!(matches[1].1, keypoints2[1]);
        assert_eq!(matches[2].0, keypoints1[2]);
        assert_eq!(matches[2].1, keypoints2[2]);
        let matches =
            super::match_features(&keypoints1, &descriptors1, &keypoints2, &descriptors2, 100);
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].0, keypoints1[0]);
        assert_eq!(matches[0].1, keypoints2[0]);
        assert_eq!(matches[1].0, keypoints1[1]);
        assert_eq!(matches[1].1, keypoints2[1]);
        assert_eq!(matches[2].0, keypoints1[2]);
        assert_eq!(matches[2].1, keypoints2[2]);
    }

    #[test]
    fn test_hamming_distance() {
        let bytes1 = [0b00000000, 0b00000000, 0b00000000, 0b00000000];
        let bytes2 = [0b00000000, 0b00000000, 0b00000000, 0b00000000];
        assert_eq!(super::hamming_distance(&bytes1, &bytes2), 0);

        let bytes1 = [0b00000000, 0b00000000, 0b00000000, 0b00000000];
        let bytes2 = [0b00000000, 0b00000000, 0b00000000, 0b00000001];
        assert_eq!(super::hamming_distance(&bytes1, &bytes2), 1);

        let bytes1 = [0b00000000, 0b00000000, 0b00000000, 0b00000000];
        let bytes2 = [0b00000000, 0b00000000, 0b00000000, 0b00000010];
        assert_eq!(super::hamming_distance(&bytes1, &bytes2), 1);

        let bytes1 = [0b00000000, 0b00000000, 0b00000000, 0b00000000];
        let bytes2 = [0b00000000, 0b00000000, 0b00000000, 0b00000011];
        assert_eq!(super::hamming_distance(&bytes1, &bytes2), 2);

        let bytes1 = [0b00000000, 0b00000000, 0b00000000, 0b00000000];
        let bytes2 = [0b00000000, 0b00000000, 0b00000000, 0b00000100];
        assert_eq!(super::hamming_distance(&bytes1, &bytes2), 1);

        let bytes1 = [0b00000000, 0b00000000, 0b00000000, 0b00000000];
        let bytes2 = [0b00000000, 0b00000000, 0b00000000, 0b00000101];
        assert_eq!(super::hamming_distance(&bytes1, &bytes2), 2);
    }
}
