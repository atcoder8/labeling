mod union_find;

use crate::union_find::UnionFind;

/// Data type of label
pub type LabelType = i32;

/// It execute four-neighborhood-based labeling.
///
/// # Arguments
///
/// * `binary_image` - Binary image represented by a two-dimensional boolean vector
///
/// # Return
///
/// Labeled image (same size as `binary_image`)
///
/// # Panics
///
/// The function panics if `binary_image` is not a rectangle.
pub fn four_neighborhood_based_labeling(binary_image: &Vec<Vec<bool>>) -> Vec<Vec<LabelType>> {
    // Height of image
    let h = binary_image.len();
    assert!(h >= 1, "Height must be greater than or equal to 1.");

    // Width of image
    let w = binary_image[0].len();
    assert!(w >= 1, "Width must be greater than or equal to 1");

    let mut labeled_image = vec![vec![-1; w]; h];

    let mut uf = UnionFind::new(0);

    for i in 0..h {
        assert_eq!(binary_image[i].len(), w, "Image shape must be rectangular.");

        for j in 0..w {
            // If the current pixel is background, skip the process.
            if !binary_image[i][j] {
                continue;
            }

            // Label of the upper pixel
            let upper_label = if i > 0 && binary_image[i - 1][j] {
                labeled_image[i - 1][j]
            } else {
                -1
            };

            // Label of the left pixel
            let left_label = if j > 0 && binary_image[i][j - 1] {
                labeled_image[i][j - 1]
            } else {
                -1
            };

            if upper_label != -1 && left_label != -1 {
                // If both the upper pixel and the left pixel are in the foreground

                // Set the current label to the label of the upper pixel.
                labeled_image[i][j] = upper_label;
                // Merge labels on union-find.
                uf.merge(upper_label as usize, left_label as usize);
            } else if upper_label != -1 {
                // If only the upper pixel is in the foreground,
                // set the current label to the label of the upper pixel.
                labeled_image[i][j] = upper_label;
            } else if left_label != -1 {
                // If only the left pixel is in the foreground,
                // set the current label to the label of the left pixel.
                labeled_image[i][j] = left_label;
            } else {
                // If neither the upper pixel nor the left pixel is in the foreground,
                // assign a new label to the current pixel.
                labeled_image[i][j] = uf.get_elem_num() as LabelType;
                uf.add();
            }
        }
    }

    // Reassign labels.
    reassign_labels(&mut labeled_image, &mut uf);

    labeled_image
}

/// It execute eight-neighborhood-based labeling.
///
/// # Arguments
///
/// * `binary_image` - Binary image represented by a two-dimensional boolean vector
///
/// # Return
///
/// Labeled image (same size as `binary_image`)
///
/// # Panics
///
/// The function panics if `binary_image` is not a rectangle.
pub fn eight_neighborhood_based_labeling(binary_image: &Vec<Vec<bool>>) -> Vec<Vec<LabelType>> {
    // Height of image
    let h = binary_image.len();
    assert!(h >= 1, "Height must be greater than or equal to 1.");

    // Width of image
    let w = binary_image[0].len();
    assert!(w >= 1, "Width must be greater than or equal to 1");

    let mut uf = UnionFind::new(0);

    let mut labeled_image = vec![vec![-1; w]; h];

    for i in 0..h {
        assert_eq!(binary_image[i].len(), w, "Image shape must be rectangular.");

        for j in 0..w {
            // If the current pixel is background, skip the process.
            if !binary_image[i][j] {
                continue;
            }

            // Label of the upper left pixel
            let upper_left_label = if i > 0 && j > 0 && binary_image[i - 1][j - 1] {
                labeled_image[i - 1][j - 1]
            } else {
                -1
            };

            // Label of the upper pixel
            let upper_label = if i > 0 && binary_image[i - 1][j] {
                labeled_image[i - 1][j]
            } else {
                -1
            };

            // Label of the upper right pixel
            let upper_right_label = if i > 0 && j < w - 1 && binary_image[i - 1][j + 1] {
                labeled_image[i - 1][j + 1]
            } else {
                -1
            };

            // Label of the left pixel
            let left_label = if j > 0 && binary_image[i][j - 1] {
                labeled_image[i][j - 1]
            } else {
                -1
            };

            // Array of 4 labels
            let surrounding_labels = [upper_left_label, upper_label, upper_right_label, left_label];

            // Iterator of the one that is foreground among the four labels
            let mut foreground_labels = surrounding_labels.iter().filter(|x| **x != -1);

            if let Some(&foreground_label) = foreground_labels.next() {
                // If foreground pixels are found

                // Set the label of the current pixel to the label of that pixel.
                labeled_image[i][j] = foreground_label;
                // Merge with all other labels of foreground pixels on union-find.
                foreground_labels.for_each(|x| {
                    uf.merge(foreground_label as usize, *x as usize);
                });
            } else {
                // If no foreground pixels are found.
                // assign a new label to the current pixel.
                labeled_image[i][j] = uf.get_elem_num() as LabelType;
                uf.add();
            }
        }
    }

    // Reassign labels.
    reassign_labels(&mut labeled_image, &mut uf);

    labeled_image
}

/// It reassign labels.
///
/// # Arguments
///
/// * `labeled_image` - Tentatively labeled image
/// * `uf` - Union-Find expressing the connection between the labels
///
/// # Panics
///
/// This function panics if the height or width of the image is zero.
fn reassign_labels(labeled_image: &mut Vec<Vec<LabelType>>, uf: &mut UnionFind) {
    // Height of image
    let h = labeled_image.len();

    // Height must be greater than or equal to 1
    assert!(h >= 1);

    // Width of image
    let w = labeled_image[0].len();

    // Width must be greater than or equal to 1
    assert!(w >= 1);

    // Vec for reassigned labels
    // The i-th index stores the reassigned label for provisional label i.
    let mut reassignment_labels: Vec<Option<LabelType>> = vec![None; uf.get_elem_num()];

    // Counting the number of labels
    let mut label_cnt = 0;

    for i in 0..h {
        for j in 0..w {
            // If the current pixel is background, skip the process.
            if labeled_image[i][j] == -1 {
                continue;
            }

            // Representative value of the connected component containing the label of the current pixel
            let leader_label = uf.leader(labeled_image[i][j] as usize);
            // Reassigned label
            let reassigned_label = &mut reassignment_labels[leader_label];

            // If no label has been set for reassignment, create a new label.
            if reassigned_label.is_none() {
                *reassigned_label = Some(label_cnt);
                label_cnt += 1;
            }

            // Reassign labels.
            labeled_image[i][j] = reassigned_label.unwrap();
        }
    }
}
