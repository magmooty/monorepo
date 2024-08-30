use genpdf::{
    elements::{Paragraph, TableLayout},
    Alignment, Document, SimplePageDecorator, Size,
};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{reshape_arabic_text, PdfGenerationContext};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ReceiptData {
    pub student_name: String,
    pub item_name: String,
    pub item_price: i32,
    pub seller_name: String,
}

pub fn gen_receipt(
    PdfGenerationContext { mut doc }: PdfGenerationContext,
    data: ReceiptData,
) -> Document {
    doc.set_font_size(9);

    let size = Size::new(60, 100);
    doc.set_paper_size(size);

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(5);
    doc.set_page_decorator(decorator);

    doc.push(Paragraph::new(reshape_arabic_text("مجموعتي")).aligned(Alignment::Center));

    let mut table = TableLayout::new(vec![1, 1]);

    table
        .row()
        .element(
            Paragraph::new(reshape_arabic_text(data.student_name.as_str()))
                .aligned(Alignment::Right),
        )
        .element(Paragraph::new(reshape_arabic_text("اسم الطالب")).aligned(Alignment::Right))
        .push()
        .expect("Invalid table row");

    table
        .row()
        .element(
            Paragraph::new(reshape_arabic_text(data.item_name.as_str())).aligned(Alignment::Right),
        )
        .element(Paragraph::new(reshape_arabic_text("المدفوع")).aligned(Alignment::Right))
        .push()
        .expect("Invalid table row");

    table
        .row()
        .element(
            Paragraph::new(reshape_arabic_text(data.item_price.to_string().as_str()))
                .aligned(Alignment::Right),
        )
        .element(Paragraph::new(reshape_arabic_text("المبلغ")).aligned(Alignment::Right))
        .push()
        .expect("Invalid table row");

    doc.push(table);

    doc.push(
        Paragraph::new(reshape_arabic_text(data.seller_name.as_str())).aligned(Alignment::Center),
    );

    doc
}
