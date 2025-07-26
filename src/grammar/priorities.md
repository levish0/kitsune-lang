| 우선순위 | 연산자 / 표현식                        | 문법 항목                | 결합 방향성 | 설명                 |
|:----:|:---------------------------------|:---------------------|:------:|:-------------------|
|  1   | 리터럴, 변수, 함수 호출, `()`             | `Term`               |  N/A   | 가장 기본적인 표현 단위      |
|  2   | `-`, `+`, `!` (단항)               | `UnaryExpr`          | Right  | 단항 연산자 (부호, 논리 부정) |
|  3   | `*`, `/`, `%`                    | `MultiplicativeExpr` |  Left  | 곱셈, 나눗셈, 나머지       |
|  4   | `+`, `-`                         | `AdditiveExpr`       |  Left  | 덧셈, 뺄셈             |
|  5   | `<<`, `>>`                       | `BitShiftExpr`       |  Left  | 비트 시프트             |
|  6   | `==`, `!=`, `<`, `<=`, `>`, `>=` | `ComparisonExpr`     |  Left  | 비교 연산              |
|  7   | `&`                              | `BitAndExpr`         |  Left  | 비트 AND             |
|  8   | `^`                              | `BitXorExpr`         |  Left  | 비트 XOR             |
|  9   | `\|`                             | `BitOrExpr`          |  Left  | 비트 OR              |
|  10  | `&&`                             | `LogicalAndExpr`     |  Left  | 논리 AND             |
|  11  | `\|\|`                           | `LogicalOrExpr`      |  Left  | 논리 OR              |
|  12  | `..`, `..=`                      | `RangeExpr`          |  Left  | 범위 표현식             |
