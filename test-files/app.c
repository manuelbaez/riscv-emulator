int main()
{
    int a = 5;
    int b = 2;
    if (a != b)
    {
        b = 3;
    }
    if (a == b)
    {
        a = 5;
    }
    if (a < b)
    {
        a = 7;
    }
    if (a >= b)
    {
        a = 8;
    }
    int c = calculate();
    return a + b + c;
}

int calculate()
{
    int var_a = 10;
    int var_b = 20;
    return var_a + var_b;
}