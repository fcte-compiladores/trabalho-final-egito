int soma(int a, int b) {
    return a + b;
}

int multiplica(int x, int y) {
    int resultado = x * y;
    return resultado;
}

int main() {
    int num1 = 9;
    int num2 = 5;
    int resultado = soma(num1, num2);
    int produto = multiplica(resultado, 2);
    return produto;
}
