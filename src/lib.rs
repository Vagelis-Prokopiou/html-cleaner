#![allow(clippy::needless_return)]

use std::vec;

pub fn ammonia_init() -> ammonia::Builder<'static> {
    return ammonia::Builder::empty();
}

/// Extra regexes for various replacements.
pub fn get_replacement_regexes() -> Vec<(regex::Regex, String)> {
    let re_multiple_spaces = regex::Regex::new(r"\s+").unwrap();
    let re_space_at_the_end = regex::Regex::new(r"\s$").unwrap();
    return vec![
        (re_multiple_spaces, " ".to_string()),
        (re_space_at_the_end, "".to_string()),
    ];
}

pub fn remove_html(ammonia: &ammonia::Builder, regexes: &Vec<(regex::Regex, String)>, value: &str) -> String {
    let mut result = ammonia
        .clean(value).to_string()
        .replace("&nbsp;", " ");
    for regex in regexes {
        let _regex = &regex.0;
        let replacement = &regex.1;
        result = _regex.replace_all(&result, replacement).to_string();
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_remove_html() {
        let ammonia = ammonia_init();
        let replacement_regexes = get_replacement_regexes();
        let cases = vec![
            (r##"<b style=""><font color="#ffffffff">5. Autoridades             Públicos &nbsp; &nbsp;</font></b>"##, "5. Autoridades Públicos"),
            (r##"<b style=""><font color="#ffffffff">5. Autoridades e Funcionários Públicos</font></b>"##, "5. Autoridades e Funcionários Públicos"),
            (r##"3.        Favor descrever quaisquer transferências de dados que sua Empresa do Grupo esteja executando. Escolha a fonte dos dados à esquerda (coluna "Fonte de Transferênvia de Dados") e o(s) destino(s) à direita <br> ("Destino Transferência de Dados")"##, "3. Favor descrever quaisquer transferências de dados que sua Empresa do Grupo esteja executando. Escolha a fonte dos dados à esquerda (coluna \"Fonte de Transferênvia de Dados\") e o(s) destino(s) à direita (\"Destino Transferência de Dados\")"),
            (r##"<span style="color: rgb(0, 0, 0); font-family: tahoma, arial, verdana, sans-serif; font-size: 12px;"><b>El Compliance es un componente integral de la cultura corporativa. </b></span><div style=""><div style="color: rgb(0, 0, 0); font-family: tahoma, arial, verdana, sans-serif; font-size: 12px;"><font face="tahoma, arial, verdana, sans-serif"><br></font></div><div style="color: rgb(0, 0, 0); font-family: tahoma, arial, verdana, sans-serif; font-size: 12px;"><font face="tahoma, arial, verdana, sans-serif">Esperamos que nuestras empresas TKE lleven a cabo sus negocios con los más altos estándares éticos, adhiriéndose a todas las leyes, normas y reglamentos. </font></div><div style="color: rgb(0, 0, 0); font-family: tahoma, arial, verdana, sans-serif; font-size: 12px;"><font face="tahoma, arial, verdana, sans-serif">Como parte del proceso de Gestión de Riesgos de Cumplimiento, le rogamos que rellene todos los cuestionarios disponibles en las secciones. </font></div><div style="color: rgb(0, 0, 0); font-family: tahoma, arial, verdana, sans-serif; font-size: 12px;"><font face="tahoma, arial, verdana, sans-serif"><br></font></div><div style=""><font face="tahoma, arial, verdana, sans-serif" style=""><div style="">La realización de la evaluación de riesgos tiene como objetivo evaluar el riesgo del posible negocio. </div><div style="">Toda la información recopilada se utiliza con fines de evaluación, para valorar los riesgos potenciales, definir las medidas correctoras e identificar las Empresas TKE con problemas de integridad existentes. </div><div style=""><br></div><div style="">Si tiene preguntas al rellenar el cuestionario, por favor [Mail_Link].<br></div></font></div></div>"##, "El Compliance es un componente integral de la cultura corporativa. Esperamos que nuestras empresas TKE lleven a cabo sus negocios con los más altos estándares éticos, adhiriéndose a todas las leyes, normas y reglamentos. Como parte del proceso de Gestión de Riesgos de Cumplimiento, le rogamos que rellene todos los cuestionarios disponibles en las secciones. La realización de la evaluación de riesgos tiene como objetivo evaluar el riesgo del posible negocio. Toda la información recopilada se utiliza con fines de evaluación, para valorar los riesgos potenciales, definir las medidas correctoras e identificar las Empresas TKE con problemas de integridad existentes. Si tiene preguntas al rellenar el cuestionario, por favor [Mail_Link]."),
        ];
        for (input, expected) in cases {
            let actual = remove_html(&ammonia, &replacement_regexes, input);
            assert_eq!(actual, expected);
        }
    }
}
