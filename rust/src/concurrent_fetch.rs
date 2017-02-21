use rayon::prelude::*;
use futures::future;
use futures::future::{Future, FutureResult};
use std::io;

#[derive(Debug)]
pub struct Document {
    id: i64,
}


/* futures */

fn fetch_document_future(id: i64) -> FutureResult<Document, io::Error> {
    future::ok(Document { id: id })
}

pub fn fetch_documents_w_futures(ids: &[i64]) -> Result<Vec<Document>, io::Error> {
    let results = ids.iter()
        .map(|&id| fetch_document_future(id));
    future::join_all(results).wait()
}


/* parallel iterators */

fn fetch_document(id: i64) -> Result<Document, io::Error> {
    Ok(Document { id: id })
}

pub fn fetch_documents_w_par_iter(ids: &[i64]) -> Result<Vec<Document>, io::Error> {
    let results = ids.par_iter()
        .map(|&id| fetch_document(id))
        .weight_max()  // force worker batch size to 1, so that every fetch is really parallel
        .collect::<Vec<_>>();
    results.into_iter().collect() // Convert from `Vec<Result<T, E>>` to `Result<Vec<T>, E>`
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    fn fetches_documents(result: Result<Vec<Document>, io::Error>) {
        let docs = result.expect("fetch was successful");
        assert_eq!(3, docs.len());
    }

    #[test]
    fn fetches_documents_using_futures() {
        let ids = vec![1, 2, 3];
        let result = fetch_documents_w_futures(&ids);
        fetches_documents(result);
    }

    #[test]
    fn fetches_documents_using_par_iter() {
        let ids = vec![1, 2, 3];
        let result = fetch_documents_w_par_iter(&ids);
        fetches_documents(result);
    }
}
