use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use crate::ast::*;
use crate::ir_parser;

#[pyfunction]
fn parse_ir(py: Python, input: &str) -> PyResult<PyObject> {
    let result = ir_parser::parse_ir(input)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Parse error: {}", e)))?;
    
    program_to_py(py, &result)
}

fn program_to_py(py: Python, program: &Program) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("type", "Program")?;
    
    let functions = PyList::empty(py);
    for func in &program.func_defs {
        functions.append(function_def_to_py(py, func)?)?;
    }
    dict.set_item("functions", functions)?;
    
    Ok(dict.into())
}

fn function_def_to_py(py: Python, func: &FunctionDef) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("type", "FunctionDef")?;
    dict.set_item("name", &func.name)?;
    
    let params = PyList::empty(py);
    for (param_name, param_type) in &func.params {
        let param_dict = PyDict::new(py);
        param_dict.set_item("name", param_name)?;
        param_dict.set_item("type", type_to_py(py, param_type)?)?;
        params.append(param_dict)?;
    }
    dict.set_item("parameters", params)?;
    
    dict.set_item("return_type", type_to_py(py, &func.return_type)?)?;
    
    let annotations = PyList::empty(py);
    for ann in &func.annotations {
        annotations.append(annotation_to_py(py, ann)?)?;
    }
    dict.set_item("annotations", annotations)?;
    
    if !func.requires.is_empty() {
        let requires_list = PyList::empty(py);
        for req in &func.requires {
            requires_list.append(expression_to_py(py, req)?)?;
        }
        dict.set_item("requires", requires_list)?;
    }
    
    if !func.ensures.is_empty() {
        let ensures_list = PyList::empty(py);
        for ens in &func.ensures {
            ensures_list.append(expression_to_py(py, ens)?)?;
        }
        dict.set_item("ensures", ensures_list)?;
    }
    
    dict.set_item("body", expression_to_py(py, &func.body)?)?;
    
    Ok(dict.into())
}

fn type_to_py(py: Python, ty: &Type) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    
    match ty {
        Type::Basic(basic) => {
            dict.set_item("kind", "Basic")?;
            dict.set_item("name", &basic.name)?;
        }
        Type::Option(opt) => {
            dict.set_item("kind", "Option")?;
            dict.set_item("inner", type_to_py(py, &opt.inner)?)?;
        }
        Type::Result(result) => {
            dict.set_item("kind", "Result")?;
            dict.set_item("ok", type_to_py(py, &result.ok_type)?)?;
            dict.set_item("err", type_to_py(py, &result.err_type)?)?;
        }
        Type::List(list) => {
            dict.set_item("kind", "List")?;
            dict.set_item("inner", type_to_py(py, &list.element_type)?)?;
        }
        Type::Tuple(tuple) => {
            dict.set_item("kind", "Tuple")?;
            let elements = PyList::empty(py);
            for t in &tuple.element_types {
                elements.append(type_to_py(py, t)?)?;
            }
            dict.set_item("elements", elements)?;
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
                format!("Type not yet implemented: {:?}", ty)
            ));
        }
    }
    
    Ok(dict.into())
}

fn annotation_to_py(py: Python, ann: &Annotation) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("name", &ann.name)?;
    
    let args = PyDict::new(py);
    for (key, value) in &ann.args {
        args.set_item(key, value)?;
    }
    dict.set_item("args", args)?;
    
    Ok(dict.into())
}

fn literal_to_py(py: Python, lit: &LiteralValue) -> PyResult<PyObject> {
    match lit {
        LiteralValue::Int(n) => Ok(n.to_object(py)),
        LiteralValue::Float(f) => Ok(f.to_object(py)),
        LiteralValue::Bool(b) => Ok(b.to_object(py)),
        LiteralValue::String(s) => Ok(s.to_object(py)),
        LiteralValue::Unit => Ok(py.None()),
    }
}

fn expression_to_py(py: Python, expr: &Expr) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    
    match expr {
        Expr::Literal(lit) => {
            dict.set_item("type", "Literal")?;
            dict.set_item("value", literal_to_py(py, &lit.value)?)?;
        }
        Expr::Variable(var) => {
            dict.set_item("type", "Variable")?;
            dict.set_item("name", &var.name)?;
        }
        Expr::BinaryOp(binop) => {
            dict.set_item("type", "BinaryOp")?;
            dict.set_item("op", &binop.op)?;
            dict.set_item("left", expression_to_py(py, &binop.left)?)?;
            dict.set_item("right", expression_to_py(py, &binop.right)?)?;
        }
        Expr::Application(app) => {
            dict.set_item("type", "Application")?;
            dict.set_item("function", expression_to_py(py, &app.func)?)?;
            dict.set_item("argument", expression_to_py(py, &app.arg)?)?;
        }
        Expr::If(if_expr) => {
            dict.set_item("type", "If")?;
            dict.set_item("condition", expression_to_py(py, &if_expr.condition)?)?;
            dict.set_item("then", expression_to_py(py, &if_expr.then_branch)?)?;
            dict.set_item("else", expression_to_py(py, &if_expr.else_branch)?)?;
        }
        Expr::Let(let_expr) => {
            dict.set_item("type", "Let")?;
            dict.set_item("name", &let_expr.var_name)?;
            dict.set_item("value", expression_to_py(py, &let_expr.value)?)?;
            dict.set_item("body", expression_to_py(py, &let_expr.body)?)?;
        }
        Expr::Match(match_expr) => {
            dict.set_item("type", "Match")?;
            dict.set_item("scrutinee", expression_to_py(py, &match_expr.scrutinee)?)?;
            
            let cases_list = PyList::empty(py);
            for (pattern, body) in &match_expr.arms {
                let case_dict = PyDict::new(py);
                case_dict.set_item("pattern", pattern_to_py(py, pattern)?)?;
                case_dict.set_item("body", expression_to_py(py, body)?)?;
                cases_list.append(case_dict)?;
            }
            dict.set_item("cases", cases_list)?;
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
                format!("Expression type not yet implemented: {:?}", expr)
            ));
        }
    }
    
    Ok(dict.into())
}

fn pattern_to_py(py: Python, pattern: &Pattern) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    
    match pattern {
        Pattern::Literal(lit_pat) => {
            dict.set_item("type", "Literal")?;
            dict.set_item("value", literal_to_py(py, &lit_pat.value)?)?;
        }
        Pattern::Variable(var_pat) => {
            dict.set_item("type", "Variable")?;
            dict.set_item("name", &var_pat.name)?;
        }
        Pattern::Wildcard(_) => {
            dict.set_item("type", "Wildcard")?;
        }
        Pattern::Constructor(ctor_pat) => {
            dict.set_item("type", "Constructor")?;
            dict.set_item("name", &ctor_pat.name)?;
            
            let args_list = PyList::empty(py);
            for arg in &ctor_pat.args {
                args_list.append(pattern_to_py(py, arg)?)?;
            }
            dict.set_item("args", args_list)?;
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
                format!("Pattern type not yet implemented: {:?}", pattern)
            ));
        }
    }
    
    Ok(dict.into())
}

#[pymodule]
fn pole_compiler(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_ir, m)?)?;
    Ok(())
}
