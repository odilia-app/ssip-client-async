use lingua::DetectionResult;
use lingua::Language::{English, Spanish, Chinese};
use lingua::LanguageDetectorBuilder;

fn main() {
    let languages = vec![English, Chinese];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    let sentence = "Hello my name is Joe. 你好世界";

    let results: Vec<DetectionResult> = detector.detect_multiple_languages_of(sentence);
    
    println!("{:?}", results);

    if let [first, second] = &results[..] {
        assert_eq!(first.language(), English);
        assert_eq!(
            &sentence[first.start_index()..first.end_index()],
            "Hello my name is Joe."
        );

        assert_eq!(second.language(), Spanish);
        assert_eq!(
            &sentence[second.start_index()..second.end_index()],
            "你好世界"
        );
    } 
}