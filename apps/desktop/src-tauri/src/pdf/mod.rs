use arabic_reshaper::arabic_reshape;
use genpdf::{
    error::Error,
    fonts::{FontData, FontFamily},
    Document,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use specta::Type;
use unicode_bidi::BidiInfo;

mod receipt;

pub use receipt::ReceiptData;

/// Reshape Arabic text to be displayed correctly in PDFs.
pub fn reshape_arabic_text(text: &str) -> String {
    let reshaped_text = arabic_reshape(text);
    let bidi = BidiInfo::new(reshaped_text.as_str(), None);
    let para = &bidi.paragraphs[0];
    let line = para.range.clone();
    let display_text = bidi.reorder_line(para, line);
    display_text.to_string()
}

pub struct PdfGenerationContext {
    pub doc: genpdf::Document,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum Report {
    Receipt(ReceiptData),
}

static FONT_FAMILY: Lazy<FontFamily<FontData>> = Lazy::new(|| {
    genpdf::fonts::from_files("./fonts", "NotoKufiArabic", None)
        .expect("Failed to load font family")
});

/// Generate a PDF file from a report.
///
/// ```rust
/// pdf::generate_pdf(
///     Report::Receipt(ReceiptData {
///         student_name: "زياد طارق".to_string(),
///         item_name: "شهر ٨".to_string(),
///         item_price: 150,
///         seller_name: "كريم جابر".to_string(),
///     }),
///     "./receipt.pdf".to_string(),
/// )
/// .await
/// .unwrap();
/// ```
pub async fn generate_pdf(report: Report, file_path: String) -> Result<(), Error> {
    tokio::task::spawn_blocking(move || -> Result<(), Error> {
        let context = PdfGenerationContext {
            doc: genpdf::Document::new(FONT_FAMILY.clone()),
        };

        let doc: Document;

        match report {
            Report::Receipt(data) => {
                doc = receipt::gen_receipt(context, data);
            }
        }

        doc.render_to_file(file_path)?;

        Ok(())
    })
    .await
    .unwrap()?;

    Ok(())
}
