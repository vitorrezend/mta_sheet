use crate::components::Callback;

/// Configurações para compressão e redimensionamento de imagem no cliente (Canvas HTML5)
#[derive(Clone, Copy, Debug)]
pub struct ImageCompressionOptions {
    pub max_width: u32,
    pub max_height: u32,
    pub quality: f64, // 0.0 a 1.0 (ex: 0.82)
}

impl Default for ImageCompressionOptions {
    fn default() -> Self {
        Self {
            max_width: 1000,
            max_height: 1000,
            quality: 0.82,
        }
    }
}

impl ImageCompressionOptions {
    pub fn portrait() -> Self {
        Self {
            max_width: 800,
            max_height: 800,
            quality: 0.82,
        }
    }

    pub fn chart_or_map() -> Self {
        Self {
            max_width: 1400,
            max_height: 1400,
            quality: 0.82,
        }
    }
}

/// Comprime um arquivo `web_sys::File` no navegador usando Canvas e exporta para formato WebP otimizado em Base64 Data URL.
#[allow(unused_variables)]
pub fn compress_image_file_to_webp(
    file: &web_sys::File,
    options: ImageCompressionOptions,
    on_complete: Callback<Result<String, String>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;
        use web_sys::{HtmlCanvasElement, HtmlImageElement, CanvasRenderingContext2d, Url};

        let object_url = match Url::create_object_url_with_blob(file) {
            Ok(u) => u,
            Err(_) => {
                on_complete.call(Err("Falha ao criar URL de objeto para a imagem".to_string()));
                return;
            }
        };

        let img = match HtmlImageElement::new() {
            Ok(i) => i,
            Err(_) => {
                let _ = Url::revoke_object_url(&object_url);
                on_complete.call(Err("Falha ao instanciar elemento de imagem".to_string()));
                return;
            }
        };

        let obj_url_clone = object_url.clone();
        let on_complete_success = on_complete.clone();
        let on_complete_error = on_complete.clone();
        let img_clone = img.clone();

        let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let _ = Url::revoke_object_url(&obj_url_clone);
            let natural_w = img_clone.natural_width() as f64;
            let natural_h = img_clone.natural_height() as f64;

            if natural_w <= 0.0 || natural_h <= 0.0 {
                on_complete_success.call(Err("Dimensões de imagem inválidas".to_string()));
                return;
            }

            // Calcula proporções máximas mantendo o aspecto original
            let max_w = options.max_width as f64;
            let max_h = options.max_height as f64;

            let (target_w, target_h) = if natural_w > max_w || natural_h > max_h {
                let ratio = (max_w / natural_w).min(max_h / natural_h);
                ((natural_w * ratio).round().max(1.0), (natural_h * ratio).round().max(1.0))
            } else {
                (natural_w, natural_h)
            };

            let doc = match web_sys::window().and_then(|w| w.document()) {
                Some(d) => d,
                None => {
                    on_complete_success.call(Err("Documento não encontrado".to_string()));
                    return;
                }
            };

            let canvas: HtmlCanvasElement = match doc.create_element("canvas").ok().and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok()) {
                Some(c) => c,
                None => {
                    on_complete_success.call(Err("Falha ao criar canvas de compressão".to_string()));
                    return;
                }
            };

            canvas.set_width(target_w as u32);
            canvas.set_height(target_h as u32);

            let ctx: CanvasRenderingContext2d = match canvas.get_context("2d").ok().flatten().and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok()) {
                Some(c) => c,
                None => {
                    on_complete_success.call(Err("Falha ao obter contexto 2D do canvas".to_string()));
                    return;
                }
            };

            ctx.set_image_smoothing_enabled(true);

            if let Err(e) = ctx.draw_image_with_html_image_element_and_dw_and_dh(&img_clone, 0.0, 0.0, target_w, target_h) {
                on_complete_success.call(Err(format!("Falha ao desenhar no canvas: {:?}", e)));
                return;
            }

            let quality_js = wasm_bindgen::JsValue::from_f64(options.quality);
            // Tenta exportar para image/webp
            match canvas.to_data_url_with_type_and_encoder_options("image/webp", &quality_js) {
                Ok(data_url) => {
                    log::info!("Imagem comprimida com sucesso para WebP ({}x{}px, {:.0}% qualidade)", target_w, target_h, options.quality * 100.0);
                    on_complete_success.call(Ok(data_url));
                }
                Err(_) => {
                    // Fallback para JPEG caso o navegador não suporte exportação WebP no canvas
                    match canvas.to_data_url_with_type_and_encoder_options("image/jpeg", &quality_js) {
                        Ok(jpeg_url) => {
                            log::info!("Imagem comprimida para JPEG (fallback)");
                            on_complete_success.call(Ok(jpeg_url));
                        }
                        Err(err) => {
                            on_complete_success.call(Err(format!("Falha ao codificar imagem: {:?}", err)));
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        let obj_url_err = object_url.clone();
        let onerror = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let _ = Url::revoke_object_url(&obj_url_err);
            on_complete_error.call(Err("Falha ao decodificar a imagem selecionada".to_string()));
        }) as Box<dyn FnMut(_)>);

        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onload.forget();
        onerror.forget();
        img.set_src(&object_url);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        on_complete.call(Err("Compressão suportada apenas no navegador (WASM)".to_string()));
    }
}
