#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextActionPaynowDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,

    /// The image_url_png string used to render QR code.
    pub image_url_png: String,

    /// The image_url_svg string used to render QR code.
    pub image_url_svg: String,
}
