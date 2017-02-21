#[derive(Debug)]
pub struct Document {
    id:          i64,
    is_archived: bool,
    title:       String,
}

pub fn latest_titles(docs: &[Document], count: usize) -> Vec<&str> {
    docs.iter()
        .filter(|ref doc| !doc.is_archived)
        .map(|ref doc| doc.title.as_str())
        .take(count)
        .collect()
}

#[test]
fn gets_titles() {
    let docs = vec![
        Document { id: 1, is_archived: false, title: "1st doc".to_string() },
        Document { id: 2, is_archived: true,  title: "2nd doc".to_string() },
        Document { id: 3, is_archived: false, title: "3rd doc".to_string() },
        Document { id: 4, is_archived: false, title: "4th doc".to_string() },
    ];
    let latest = latest_titles(&docs, 2);
    assert_eq!(2, latest.len());
    assert_eq!("1st doc", latest[0]);
    assert_eq!("3rd doc", latest[1]);
}
