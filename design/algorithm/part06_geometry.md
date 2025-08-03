# Введение

![Alt text to image](/assets/algorithm/image_scheme.png)

Схема крана влкючается в себя:
- Две стрелы: 
  - **main boom** с длиной - *l_mainboom*
  - **knuckle boom** с длиной - *l_knuckleboom*
- Пять блоков с координатами `[(X1;Y1), (X2;Y2), (X3;Y3), (X4;Y4), (X5;Y0)]`
- Барабан лебедки с координатами `[(X0;Y0)]`

# Расчет координат 
Для создания уравнений координат блоков (голубые окружности на схеме) на основе входных углов двух секций стрелы (пусть это будут углы α₁ и α₂), нужно сделать следующее:

![Alt text to image](/assets/algorithm/image_scheme2.png)

## Уравнения координат:

### 1. Координаты точки A (конец первой секции стрелы):

$$X_A = l_{mainboom} \cdot cos(a_1)\\ Y_A = l_{mainboom} \cdot sin(a_1)$$

### 2. Координаты точки B (конец хобота):

- Сначала суммарный угол: 
$$\theta = 180 - (a_1 + a_2)$$
- Тогда:
$$
X_B = X_A + l_{knuckleboom} \cdot cos (\theta) = l_{mainboom} \cdot cos(a_1) + l_{knuckleboom} \cdot cos (\theta) \\
Y_B = Y_A + l_{knuckleboom} \cdot sin (\theta) = l_{mainboom} \cdot sin(a_1) + l_{knuckleboom} \cdot sin (\theta)
$$

### 3. Расчет координаты блока 1 (X1, Y1)

$$
X1 = l_{mainboom} \cdot cos(a_1) - l1_{vertical} \cdot sin(a_1) + l1_{horisontal} \cdot cos(a_1) \\
Y1 = l_{mainboom} \cdot sin(a_1) + l1_{vertical} \cdot cos(a_1) + l1_{horisontal} \cdot sin(a_1)
$$

### 4. Расчет координаты блока 2 (X2, Y2)
$$a_3 = 180 - a_1 - a_2$$

$$
X2 = l_{mainboom} \cdot cos(a_1) + l2_{vertical} \cdot cos(a_4) + l2_{horisontal} \cdot sin(a_3) \\
Y2 = l_{mainboom} \cdot sin(a_1) - l2_{vertical} \cdot sin(a_3) + l2_{horisontal} \cdot cos(a_3)
$$

### 4. Расчет координаты блока 3 (X3, Y3)


### 5. Расчет координаты блока 4 (X4, Y4)


### 6. Расчет координаты блока 5 (X5, Y5)

