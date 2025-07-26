use crate::ast::ast::*;
use std::collections::{HashMap, HashSet};

pub trait Visitor<T> {
    fn visit_statement(&mut self, stmt: &Statement) -> T;
    fn visit_expression(&mut self, expr: &Expression) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_parameter(&mut self, param: &Parameter) -> T;
}
#[derive(Debug, Clone)]
pub enum ParameterMode {
    ByValue,     // 값으로 전달 (복사)
    ByReference, // 참조로 전달 (&T)
    ByMutRef,    // 가변 참조로 전달 (&mut T)
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<(String, ParameterMode)>,
    pub modifies_params: HashSet<String>, // 매개변수를 수정하는지
    pub calls: HashSet<String>,           // 이 함수가 호출하는 다른 함수들
}
// 변수 정보를 추적하는 구조체
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,                           // 원본 이름
    pub rust_name: String,                      // Rust에서 사용할 이름
    pub is_mutable: bool,                       // 재할당이 일어나는지
    pub is_reassigned: bool,                    // 실제로 재할당되었는지
    pub scope_level: usize,                     // 스코프 레벨
    pub var_type: Option<String>,               // 변수 타입 (추론된)
    pub first_assignment: bool,                 // 첫 번째 할당인지
    pub modified_by_functions: HashSet<String>, // 이 변수를 수정하는 함수들
    pub passed_to_functions: HashSet<String>,   // 이 변수가 전달되는 함수들
}

// 스코프 정보
#[derive(Debug)]
pub struct SemanticAnalyzer {
    pub scopes: Vec<HashMap<String, VariableInfo>>,
    pub current_scope: usize,
    pub functions: HashMap<String, FunctionSignature>,
    pub call_graph: HashMap<String, HashSet<String>>, // 함수 호출 그래프
    pub reverse_call_graph: HashMap<String, HashSet<String>>, // 역방향 호출 그래프 (callee -> callers)
    pub current_function: Option<String>,
    pub reassignments: HashSet<String>,

    // 분석 상태
    pub analysis_complete: bool,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            current_scope: 0,
            functions: HashMap::new(),
            call_graph: HashMap::new(),
            reverse_call_graph: HashMap::new(), // 역방향 그래프 추가
            current_function: None,
            reassignments: HashSet::new(),
            analysis_complete: false,
        }
    }

    // 1단계: 함수 시그니처 수집
    pub fn collect_function_signatures(&mut self, statements: &[Statement]) {
        for stmt in statements {
            self.collect_function_signature_recursive(stmt);
        }
    }

    pub fn collect_function_signature_recursive(&mut self, stmt: &Statement) {
        todo!()
    }
}
