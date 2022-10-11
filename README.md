# Ray Tracing In One Weekend

This is a Rust implementation of Peter Shirley's "Ray Tracing in One Weekend". Currently this implementation covers the first [book](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## Extra features

- [ ] Parallelised rendering using rayon

Sequential time: 311s
Parallel time 1: 86s
Parallel time 2: 74s
Parallel time 3: 71s
Execution speedup: 4.4x

Nested x1 parallel time 1: 78s
Nested x1 parallel time 2: 73s
Nested x1 parallel time 3: 72s
Nested x2 parallel time: exceeded sequential time
