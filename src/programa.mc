int soma(int a, int b) {
    return a + b;
}

int multiplica(int x, int y) {
    int resultado = x * y;
    return resultado;
}

int fibonacci(int n) {
    int a = 0;
    int b = 1;
    int temp = a + b;
    return temp;
}

int main() {
    int num1 = 10;
    int num2 = 5;
    int resultado = soma(num1, num2);
    int produto = multiplica(resultado, 2);
    int fib = fibonacci(5);
    int final = produto + fib;
    return final;
}
