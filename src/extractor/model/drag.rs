use dioxus::{
    html::{FileData, HasFileData},
    prelude::{FormEvent, *},
};
use std::pin::Pin;
pub fn create_upload_handler<T>(
    read_files: impl Fn(Vec<FileData>) -> T + 'static + Clone,
) -> impl Fn(FormEvent) -> Pin<Box<dyn std::future::Future<Output = ()>>> {
    let read_files2 = read_files.clone();
    move |evt: FormEvent| {
        Box::pin({
            let value = read_files2.clone();
            async move {
                let file_list = evt.files();
                if !file_list.is_empty() {
                    value(file_list);
                }
            }
        })
    }
}
pub fn create_drop_handler<T>(
    read_files: impl Fn(Vec<FileData>) -> T + 'static + Clone,
) -> impl Fn(DragEvent) -> Pin<Box<dyn std::future::Future<Output = ()>>> {
    let read_files2 = read_files.clone();
    move |evt: DragEvent| {
        Box::pin({
            let value = read_files2.clone();
            async move {
                let file_list = evt.files();
                if !file_list.is_empty() {
                    value(file_list);
                }
            }
        })
    }
}

// fn create_upload_handler<T>(
//     read_files: impl Fn(Arc<dyn FileEngine>) -> T + 'static + Clone
// ) -> impl Fn(FormEvent) -> Pin<Box<dyn std::future::Future<Output = ()>>> {
//     let read_files2= read_files.clone();
//     move |evt: FormEvent| Box::pin(async move {
//         if let Some(file_engine) = evt.files() {
//             read_files2(file_engine);
//         }
//     })
// }
