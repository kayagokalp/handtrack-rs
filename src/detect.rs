use crate::{
    tensor::{model::Model, session::Session},
    utils::{
        detection::{DetectionBox, Point, Rectangle},
        image::Image,
        opts::DetectionOptions,
    },
};
use anyhow::Result;
use tensorflow::{SessionRunArgs, Tensor};

/// Run the detection for the given `Image` and `DetectionOptions`.
pub fn detect(image: Image, opts: DetectionOptions) -> Result<Vec<DetectionBox>> {
    // Create the input tensor from `Image`.
    let input = image.tensor()?;
    // Get the detection model from the disk.
    let model = Model::from_frozen_graph()?;
    let graph = model.graph();
    // Create the session for detection.
    let session = Session::from_model(&model)?;

    let image_tensor = graph.operation_by_name_required("image_tensor")?;
    let d_boxes = graph.operation_by_name_required("detection_boxes")?;
    let d_scores = graph.operation_by_name_required("detection_scores")?;

    // Construct input and outputs in session run arguments.
    let mut args = SessionRunArgs::new();
    args.add_feed(&image_tensor, 0, &input);
    let boxes_token = args.request_fetch(&d_boxes, 0);
    let scores_token = args.request_fetch(&d_scores, 0);

    // Run the session.
    session.session().run(&mut args)?;

    // Fetch outputs.
    let boxes: Tensor<f32> = args.fetch(boxes_token)?;
    let scores: Tensor<f32> = args.fetch(scores_token)?;

    // Maximum number of hands we will find in the output.
    let max_hands = opts.max_hands;
    // Minimum score needed for classifying the box, as detected hand.
    let score_threshold = opts.score_threshold;
    let detected_boxes = detection_boxes(
        scores,
        boxes,
        score_threshold,
        max_hands,
        image.height,
        image.width,
    );

    Ok(detected_boxes)
}

/// Extracts valid boxes with given minimum score from detected boxes.
fn detection_boxes(
    scores: Tensor<f32>,
    boxes: Tensor<f32>,
    score_threshold: f32,
    max_hands: usize,
    height: u64,
    width: u64,
) -> Vec<DetectionBox> {
    let mut detection_boxes = Vec::new();
    for i in 0..max_hands {
        if scores[i] > score_threshold {
            // Output tensor has the box's points in the following format:
            // box1_left box1_right box1_top box1_bottom box2_left ...
            // So each box starts at i * 4 th index and (i*4) + 1/2/3 is also for the same box.
            let index = i * 4;
            let top = boxes[index] * height as f32;
            let left = boxes[index + 1] * width as f32;
            let bottom = boxes[index + 2] * height as f32;
            let right = boxes[index + 3] * width as f32;

            // Cast to integer points
            let left = left.floor() as u32;
            let right = right.floor() as u32;
            let top = top.floor() as u32;
            let bottom = bottom.floor() as u32;

            // Create points defining the detection box.
            let lt_point = Point::new(left, top);
            let rb_point = Point::new(right, bottom);
            // Create rectangle for the detection box.
            let rect = Rectangle::new(lt_point, rb_point);
            // Create the detection box.
            detection_boxes.push(DetectionBox::new(rect, scores[i]));
        }
    }
    detection_boxes
}

#[cfg(test)]
mod test {
    // Checks if error rate is in acceptable range.
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    use crate::{
        detect::detect,
        utils::{image::Image, opts::DetectionOptions},
    };
    use std::path::PathBuf;

    #[test]
    pub fn test_detect_single_hand() {
        // Construct image.
        let project_dir = env!("CARGO_MANIFEST_DIR");
        let project_dir = PathBuf::from(project_dir).join("test/single_hand.jpeg");
        let image = Image::from_file(project_dir).unwrap();

        // Construct detection options.
        let score_threshold = 0.7f32;
        let max_hands = 1;
        let detection_opts = DetectionOptions::new(max_hands, score_threshold);
        // Run the detection.
        let detection = detect(image, detection_opts).unwrap();
        assert!(detection.len() == 1);

        let detection_box = &detection[0];
        let lt = &detection_box.rect.lt;
        let rb = &detection_box.rect.rb;

        assert_delta!(lt.x, 221, 2);
        assert_delta!(lt.y, 65, 2);
        assert_delta!(rb.x, 368, 2);
        assert_delta!(rb.y, 235, 2);
    }

    #[test]
    pub fn test_detect_multi_hand() {
        // Construct image.
        let project_dir = env!("CARGO_MANIFEST_DIR");
        let project_dir = PathBuf::from(project_dir).join("test/multi_hand.jpeg");
        let image = Image::from_file(project_dir).unwrap();

        // Construct detection options.
        let score_threshold = 0.7f32;
        let max_hands = 2;
        let detection_opts = DetectionOptions::new(max_hands, score_threshold);
        // Run the detection.
        let detection = detect(image, detection_opts).unwrap();
        assert_eq!(detection.len(), 2);

        let detection_box = &detection[0];
        let lt = &detection_box.rect.lt;
        let rb = &detection_box.rect.rb;

        assert_delta!(lt.x, 292, 2);
        assert_delta!(lt.y, 177, 2);
        assert_delta!(rb.x, 375, 2);
        assert_delta!(rb.y, 302, 2);

        let detection_box_2 = &detection[1];
        let lt2 = &detection_box_2.rect.lt;
        let rb2 = &detection_box_2.rect.rb;

        assert_delta!(lt2.x, 38, 2);
        assert_delta!(lt2.y, 188, 2);
        assert_delta!(rb2.x, 122, 2);
        assert_delta!(rb2.y, 294, 2);
    }
}
