use indoc::formatdoc;

use crate::component_generator::ComponentGenerator;
use crate::ident::RustIdent;
use crate::stripe_object::{RequestParams, RequestSpec};

impl ComponentGenerator {
    pub fn gen_request(&mut self, data: RequestSpec) -> String {
        let func_name = data.func_name();
        let return_type = &data.returned;
        let return_type_disp =
            self.make_type_printable(return_type, RustIdent::joined("Returned", &func_name));
        let mut params = vec![("client", "&crate::Client".to_string())];
        for param in &data.path_params {
            params.push((&param.name, param.rust_type.to_string()));
        }

        if let Some(param_typ) = data.params.as_type() {
            let type_disp =
                self.make_type_printable(param_typ, RustIdent::joined(&func_name, "params"));
            params.push(("params", type_disp.to_string()));
        }
        let params_body =
            params.into_iter().map(|p| format!("{}:{}", p.0, p.1)).collect::<Vec<_>>().join(",");

        let req_path = data.operation.path.trim_start_matches("/v1");

        // Parameterized request path
        let path_arg = if data.operation.path.contains('}') {
            let fmt_args = data
                .path_params
                .iter()
                .map(|p| format!("{0}={0}", p.name))
                .collect::<Vec<_>>()
                .join(",");
            format!(r#"&format!("{req_path}", {fmt_args})"#)
        } else {
            // Plain request path
            assert!(data.path_params.is_empty(), "Did not expect path params");
            format!(r#""{req_path}""#)
        };
        let body = match &data.params {
            RequestParams::Get { .. } => {
                format!("client.get_query({path_arg}, params)")
            }
            RequestParams::Post { .. } => {
                format!("client.post_form({path_arg}, params)")
            }
            RequestParams::Delete => {
                format!("client.delete({path_arg})")
            }
        };
        formatdoc!(
            r#"
        pub fn {func_name}({params_body}) -> crate::Response<{return_type_disp}> {{
            {body}
        }}
        "#
        )
    }
}
