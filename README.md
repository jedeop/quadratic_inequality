# 이차부등식
Rust를 이용해 이차부등식을 풀어봅시다!

## 사용법
```sh
cargo run "(이차부등식)"

# 빌드한 바이너리가 있다면
./quadratic_inequality "(이차부등식)"
```
### 이차부등식 입력 방법
* 제곱은 `^2`로 표현합니다.
* 띄어쓰기가 없어야 합니다.
* 부등호는 다음의 기호로 표현할 수 있습니다: `<` `<=` `≤` `>` `>=` `≥`
## 실행 모습
```sh
.\quadratic_inequality "x^2+3x-10>0"
# x^2+3x-10>0
# x < -5 OR x > 2

.\quadratic_inequality "x^2+6x+4<-4"
# x^2+6x+4<-4
# -4 < x < -2

.\quadratic_inequality "7x+10+x^2>=0"
# 7x+10+x^2>=0
# x ≤ -5 OR x ≥ -2

.\quadratic_inequality "x^2+6x+4-x<=0"
# x^2+6x+4-x<=0
# -4 ≤ x ≤ -1
```
