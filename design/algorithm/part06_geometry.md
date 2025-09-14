# Расчет координат блоков на кране

**Цель** - рассчитать глобальные координаты центров блоков, установленных на стреле крана. Расчет основан на длине и угле наклона каждой стрелы, а также на локальных координатах блоков относительно начала стрел.

Схема крана включает в себя:

- **Две стрелы:**
  - `main boom` — основная стрела, длина обозначается: $L_{boom(1)}$
  - `knuckle boom` — складная стрела, длина обозначается $L_{boom(2)}$
- **Шесть блоков:**
  - 1 блок — барабан лебедки, который стоит отдельно от стрел и его координаты будут определяться отдельно
  - 2-6 блоки — блоки на стрелах

![Alt text to image](/assets/algorithm/Crane1.png)

**Расчет подразделяется на следующие этапы:**  
1. Расчёт абсолютного угла наклона каждой стрелы
2. Определение начальной точки и конечной точки для каждой стрелы
3. Определение координаты точек блоков
4. Определение точек схода каната
5. Алгоритм учитывающий координаты блоков вне стрел
6. Определение координат крюковой подвески 
7. Реализация (Пример)

## 1. Расчёт абсолютного угла наклона каждой стрелы

Поскольку каждая следующая стрела (звено) поворачивается относительно предыдущей, её ориентация в глобальной системе координат зависит от **накопленной суммы всех предыдущих углов**.

Исходные данные:  
$$\alpha = [\alpha_1, \alpha_2, \ldots, \alpha_n]$$ — массив углов между стрелами (в градусах), а для первой стрелы между стрелой и горизонтом

Эти углы снимаются с датчика и определяют, насколько повёрнута каждая стрела **относительно предыдущей**.  
  Например:
  - $\alpha_1$ — угол основной стрелы к горизонту,
  - $\alpha_2$ — угол второй стрелы относительно первой и т.д.

![Alt text to image](/assets/algorithm/angles.png)

### Формула для абсолютного угла наклона стрелы к горизонту:

$$
\alpha_{boom_i} = \left( \sum_{i=1}^{n} \alpha_{i} \right) - 180 \cdot (i - 1)
$$

Где:
- $i$ — номер стрелы (1, 2, ...),
- $\alpha_i$ — угол между $i$-й стрелой и предыдущей (в градусах), для первой между стрелой и горизонтом.
- $\alpha_{boom_i}$ — угол $i$-й стрелы относительно горизонта.

## Общие формулы, необходимые для расчета в общем виде

**1. Формула определения координат точки в повернутой системе координат**

Если повернутая система координат повёрнута **против часовой стрелки**, то угол берётся со знаком **плюс (+)**.   Если **по часовой стрелке** — со знаком **минус (−)**.

Формулы представленные ниже - являются формулой в общем виде, и она будет использоваться в расчетах далее.

$$
X_x(X_{x1}, Y_{x1}, \alpha_{boom_i}) = X_{x1} \cdot \cos(\alpha_{boom_i}) - Y_{x1} \cdot \sin(\alpha_{boom_i})  
$$
  
$$
Y_x(X_{x1}, Y_{x1}, \alpha_{boom_i}) = X_{x1} \cdot \sin(\alpha_{boom_i}) + Y_{x1} \cdot \cos(\alpha_{boom_i})
$$

$$
XY_x =
\begin{bmatrix}
X_x \\
Y_x
\end{bmatrix}
$$

Где 
- $X_{x1}, Y_{x1}$ - координаты в неповернутой СК 
- $\alpha_{boom_i}$ - абсолютный угол между i-стрелой и горизонтом  
- $X_x, Y_x$ - координаты в повернутой СК  
- $XY_x$ - переменная с координатами точки $x$ в повернутой СК  

**2. Угол наклона прямой к горизонту**

$$
a = asin(\frac{Y_1 - Y_2}{\sqrt{(X_2 - X_1)^2 + (Y_2 - Y_1)^2}})$$

- $if \quad X_1 <= X_2$
  
$$\alpha_{horiz}(Y_1, Y_2, X_1, X_2) = a$$

- $else:$ 

$$\alpha_{horiz}(Y_1, Y_2, X_1, X_2) = 180 - a$$

Где 
- $X_{1}, Y_{1}$ - координаты начала прямой
- $X_{2}, Y_{2}$ - координаты конца прямой
- $a -$ угол наклона прямой в горизонту 

**3. Длина отрезка проходящего через 2 точки**

$$l_{section}(Y_1, Y_2, X_1, X_2)=\sqrt{(X_2 - X_1)^2 + (Y_2 - Y_1)^2}$$

**4. Нахождение расстояния от точки до прямой**

$$
l_{arm}(Y_1, Y_2, X_1, X_2, x,y) = \frac{x-X_1}{X_2-X_1} = \frac{y-Y_1}{Y_2-Y_1}
$$

Где
- $x, y$ - координаты точки 

**5.Каноническое уравнение проходящее через 2 точки**

$$
d =\frac{(Y_2 - Y_1)\cdot x - (Y_2 - Y_1)\cdot X_1 - (X_2 - X_1)\cdot y + (X_2-X_1)\cdot Y_1}{\sqrt{({Y_2 - Y_1})^2 + ({X_2 - X_1})^2}}
$$ 

где: 
- $d -$ расстояние от точки до прямой 


## 2. Определение начальной точки и конечной точки для каждой стрелы:

Для расчета координат начала стрел, $XY_{start(i)}$ необходим следующий алгоритм : 

**1. Введем две составляющие $x_0$ и $w$.**

- Переменная $w$, которая расчитывается по формуле определения координат в повернутой системе координат со следующими входными данными: 

  - $l_{3,i}$ - горизонтальное смещение точки $A_i$ стрелы относительно точки $G_{i-1}$ (относительно ГСК для первой срелы) в соответствии с рисунком.
  - $l_{4,i}$ - вертикальное смещение точки $A_i$ стрелы относительно точки $G_{i-1}$ (относительно ГСК для первой срелы) в соответствии с рисунком.
  - $\alpha'_{boom_i}$ - необходимый угол
  - Вторая составляющая - $x_0$ 

![Alt text to image](/assets/algorithm/l3_l4.png)

**2. Рассчитаем $w$**

$$w_x(l_{4,i}, -l_{3,i}, \alpha'_{boom_i} = l_{4,i} \cdot \cos(\alpha'_{boom_i}) + l_{3,i} \cdot \sin(\alpha'_{boom_i})$$

$$w_y(l_{4,i}, -l_{3,i}, \alpha'_{boom_i}) = l_{4,i} \cdot \sin(\alpha'_{boom_i}) - l_{3,i} \cdot \cos(\alpha'_{boom_i})$$


$$
w =
\begin{bmatrix}
w_x \\
w_y
\end{bmatrix}
$$

**3. Определим $x_0$ для первой и следующих стрел**

- Для **первой стрелы** ($i = 1$) начальная точка в нашем случае совпадает с ГСК:
  Для первой стрелы угол принимается за 90.

$$x_0 =[0, 0]; \quad \alpha'_{boom_i}=90^\circ$$

- Для **последующих стрел** ($i > 1$) начальная точка совпадает с концом предыдущей стрелы, то есть с точкой $G_{i-1}$:

$$x_0 = G_{i-1}; \quad \alpha'_{boom_i}=\alpha_{boom_{(i-1)}}$$

**4. Определим координаты $XY_{start(i)}$**

$$XY_{start(i)} = x_0 + w$$

**5. Рассчитаем координаты $D_i$ и $G_i$ в ГСК и определим матрицу $Т$**

Для расчета коодинат $D_i$ и $G_i$ нам необходимы следующие величины 

  - $l_{1,i}$ - **вертикальное расстояние** от точки $D_i$ до точки $A_i$ (перпендикуляр от D до оси поворота стрелы) в соответсвии с рисунком
  - $l_{2,i}$ - **горизонтальное расстояние** от точки $D_i$ до точки $A_i$ (вдоль оси стрелы) в соответсвии с рисунком

![Alt text to image](/assets/algorithm/l1_l2.png)

- Точка **D** (начало стрелы):

$$
D_i = XY_{start(i)} + XY_x(-l_{(2,i)},\quad l_{(1,i)}, \quad\alpha_{boom(i)})
$$

- Точка **G** (конец стрелы):

$$
G_i = XY_{start(i)} + XY_x(L_{boom(i)} -l_{(2,i)},\quad l_{(1,i)}, \quad \alpha_{boom(i)})
$$

- Матрица **Т** - матрица, содержащая пары точек (D, G) для каждой стрелы.

$$
T =
\begin{bmatrix}
D_i  \\
G_i
\end{bmatrix}
$$

---
> Эти точки используются для построения трансформационной матрицы $T_i$, описывающей положение стрелы $i$ в пространстве, а также для определения координат всех объектов, расположенных на этой стреле.

## 3. Определение координаты точек блоков
Для определения кооординат точек блоков, нам неуобходимы будут: 
- $K_{lFx(i,j)}$ - признак, показывающий от какой точки брать координату: от конца или начала стрелы, где $i -$ номер стрелы, $j -$ номер блока. 
- $l_{F_x(i,j)}$ - перпендикулярное расстояние от блока до оси стрелы
- $l_{F_y(i,j)}$ - продольное расстояние (вдоль стрелы) от выбранной точки $K_{lFx}$ - (D или G) до проекции точки на ось стрелы 

![Alt text to image](/assets/algorithm/Flx_Fly.png)

Алгоритм следующий: 

1. Определение  $K_{lFx(i,j)}$
   
- Если $K_{lFx(i,j)} == "D"$ , то мы берем из массива T значения из **первой** строки для i-ой стрелы и j-блока, так как туда записывали $D_i$. Для упрощения **a** - индекс строки массива, для $"D"$ **a --> 1**

- Если $K_{lFx(i,j)} == "G"$ , то мы берем из массива T значения из **второй** строки для i-ой стрелы и j-блока, так как туда записывали $G_i$ Для упрощения **a** - индекс строки массива, для $"G"$ **a --> 2**

Данный параметр будет использоваться для телескопических стрел, для остальных - данный параметр не используется.
Привязка будет к концу стрелы, так как большиство блоков находятся на конце стрелы.


Далее считаем для каждого блока координаты по общей формул

$$
XY_{block(i,j)} = XY_x(l_{F_x(i,j)}, \quad l_{F_y(i,j)}, \quad \alpha_{boom(i)}) +  G_{i}
$$

$$
XY_{block(i,j)} =
\begin{bmatrix}
X_{block(i,j)} \\
Y_{block(i,j)}
\end{bmatrix}
$$

## 4. Алгоритм определения точек схода каната на блоках 

Для определения параметров каната между блоками используется функция `Rope_btw_blocks` на вход которой необходимы следующие параметры:

```python
Rope_btw_blocks(Y1, Y2, X1, X2, D1, D2, scheme)
```
Далее рассмотрим алгоритм определения параметров в этой функции.


**1. Определение схемы (`scheme`) схода каната** --> присваивание значений $k$ и $j$ в соответсвии с рисунком и таблицей ниже. 

![Alt text to image](/assets/algorithm/схема_схода_каната.jpg)

Существую 4 схемы схода каната, в соответсвие с рисунком необходимо определить к какому типу схему относится сход каната на i-том блоке. Далее по нему присваиваются значения $k$ и $j$ , как представлено ниже.

- **Схема 1 :** *k = -1, j = 1*
- **Схема 2 :** *k = 1, j = 1*
- **Схема 3 :** *k = 1, j = -1*
- **Схема 4 :** *k = -1, j = -1*

**2. Находим длину  отрезка между двумя точками**

Для нахождения расстояния между центрами блоков необходимо применить **формулу 3** из раздела *"Общие формулы"*

$$l_{block} = l_{section}(Y_1, Y_2, X_1, X_2) $$

**3. Находим угол между центром блока и горизонтом** 

**Формула 2** из раздела *"Общие формулы"*

$$\alpha_{block} = \alpha_{horiz}(Y_1, Y_2, X_1, X_2)$$

**4. Находим угол каната**

Расчитываем угол каната по формуле 

$$\alpha_{rope} = \alpha_{block} + j \cdot asin(\frac{0.5 \cdot (D_1 + k \cdot D_2)}{l_{block}})$$

Где 
- $D_1, D_2 -$ диаметры блоков 

**5. Находим координаты**
$$X_{1block} = X_1 + j \cdot 0.5 \cdot D_1 \cdot sin(\alpha_{rope}) $$

$$Y_{1block} = Y_1 + j \cdot 0.5 \cdot D_1 \cdot cos(\alpha_{rope}) $$

$$X_{2block} = X_2 - j \cdot k \cdot 0.5 \cdot D_2 \cdot sin(\alpha_{rope}) $$

$$Y_{2block} = Y_2 - j \cdot k \cdot 0.5 \cdot D_2 \cdot cos(\alpha_{rope}) $$

**6. Находим длину каната, заключенного между точками схода**
   
$$l_{rope} = l_{section}(Y_{2block}, Y_{1block}, X_{2block}, X_{1block})$$

## 5. Алгоритм, учитывающий координаты блоков вне стрел

Для расчета координат блоков вне стрелы введем параметр $Feature_{block}$, который характеризует положение блока:

- *"{i} стрела"*
- *"Вне стрелы"*
- *"Подвеска"*

**Координаты блоков крюковой подвески**
- $if \quad Feature_{block} == {"Подвеска"}$

$$lF_x == NaN$$
**Координаты блоков внес стрел**
- $else if \quad Feature_{block} == {"Вне стрелы"}$

$$
XY_{block(i,j)} = XY_x(-l_{F_x(i,j)}, \quad l_{F_y(i,j)}, \quad {0}) +  XY_x(l_{4,1}, \quad l_{3,1}, \quad {90})
$$

**Координаты блоков на стрелах**

$$
XY_{block(i,j)} = XY_x(l_{F_x(i,j)}, \quad l_{F_y(i,j)}, \quad \alpha_{boom(i)}) +  G_{i}
$$

И далее все собирается в один массив $XY_{F_{boom}}$

![Alt text to image](/assets/algorithm/crane_with_0block.png)

## 6. Определение координат крюковой подвески (КП)
Изначально была возможность сразу задавать координаты всех блоков, однако принято решение выделить расчет КП в отдельный этап. Это обусловлено тем, что положение крюковой подвески зависит от множества факторов, включая приложенные к грузу силы, угол крена судна, длину подвеса, степень опускания крюка и другие параметры.

Разделение расчетов позволяет избежать многократного пересчета координат блоков, которые не зависят от указанных переменных, в основном цикле моделирования

Исходные данные: 
- $n_{hook}$ - номер блока, относящийся к крюковой подвеске
- $l_{hook}$ - длина подвеса 
 
$$
XY_{hook}  =
\begin{bmatrix}
(XY_{F_{boom}})_{n_{hook} - 1}+ 0.5 \cdot (D_{j})_{n_{hook} - 1} 
\\
\\
(XY_{F_{boom}})_{n_{hook} - 1} - l_{hook}
\end{bmatrix}
$$

В общем виде определяется более сложно. Далее обьединяем в координаты всех блоков и КП в общий массив $XY_{block(i,j)}$

![Alt text to image](/assets/algorithm/Hook_block.png)


## 7. Расчет каната между блоками
Для расчёта каната между блоками используется функция `rope_parameters`, которая принимает координаты центров двух блоков, их диаметры и схему схода каната.

Для определения каната между блоками, используется функция 
```python
Rope_btw_blocks = Rope_block(X_{block(i,j)}, Y_{block(i,j)}, D_{block}, scheme)
```

```python
for i in range(scheme)
Rope_btw_blocks(Y_{block(i,j)}, Y_{block(i,j+1)}, X_{block(i,j)}, X_{block(i,j+1)}, D_{block_i}, D_{block_i+1}, scheme_i)
```

## 8. Расчет длины каната между блоками 

Данный этап нужен для того, чтобы вычислить фактическую суммарную длину каната, проходящего:
- по прямолинейным участкам между блоками,
- по дугам, обхватывающим блоки,
- а также учесть запас на барабане лебёдки.
  
Алгоритм состоит из следующих этапов:

### 8.1 Длина прямолинейного участка

- Для каждого блока рассчитывается длина прямого участка каната $l_{section(i)}$ — это длина отрезка между двумя точками схода каната.

- Далее прямолинейные участки суммируются

$$l_{section_{summ}} = \sum l_{section(i)}$$


### 8.2 Рассчитывается длина дуги на блоках

Рассчитывается длина дуги на блоках $L_{sys_{arc}}$ — определяется по углу обхвата каната на блоке и радиусу блока.

Алгоритм следующий

**1. Необходимо определить угол обхвата каната**, который определяется следующим образом: 

- Если количество углов каната, (посчитанных на прошлом этапе) $\alpha_{rope}$ > 1, то 

```python 
for i in range (1, alpha_rope - 1)
```

$$
\alpha_{wrap} = \alpha_{rope_{i+1}} - \alpha_{rope_i}
$$

- в других случаях

$$\alpha_{wrap} = 0$$

то есть при $\alpha_{rope} \leq 1$ дуга не учитывается
**2. Далее определить длину дуги на блоке**

Длина дуги на блоке определяется по углу обхвата , посчитанному выше и по радиусу блока 

$$L_{arc} = \frac{\pi \cdot R \cdot \alpha_{wrap}}{180}$$
где
- $R_j = \frac {D_j}{2}$

и по этой формуле собираем массив $L_{sys_{arc}}$, где есть длина каната на каждом блоке

### 8.3 Определение длины каната на барабане

Для расчета нам необходим:
- Делительный диаметр барабана (диаметр по центрам каната 1 слоя барабана)
$$D_1 = 845 мм$$
- Количество канатов сходящих с барабана
$$a_n = 1$$
- Дополнительный запас каната на барабане сверх необходимых минимальных 3х витков
$$L_{stock} = 0 м$$
- Фактическая суммарная длина каната
$$L_{fact} = 88 м$$

$$L_{stock(total)} = 3 \cdot \pi \cdot D_1 \cdot a_n + L_{stock}$$


### 8.4 Суммируем 
$$L_{rope_(sum)} = l_{section_{summ}} + L_{sys_{arc}} + L_{stock(total)}$$

### 8.5 Вычислияем остаток на барабане 
$$L_{winch} = L_{fact} - l_{section_{summ}} - L_{sys_{arc}}$$

## Реализация (Пример)

## Входные данные

| Переменная         | Тип / Формат           | Описание                                                       |
|--------------------|------------------------|-----------------------------------------------------------------|
| $L_{boom(i)}$       | float (мм)             | Длина i-ой стрелы стрелы                                          |                                  |
| $\alpha_i$      | float (°)              | Угол между стрелами                    |        |            |
| $n$       |                int |         Количество стрел |     
| $i$     | int                 | Номер стрелы: `1` — `main boom`, `2` — `knuckle boom`          |
|$l_{1, i}$|float(мм)|**Вертикальное расстояние** от точки $D_i$ до точки $A_i$ (перпендикуляр от D до оси поворота стрелы)|
|$l_{2, i}$|float(мм)|**Горизонтальное расстояние** от точки $D_i$ до точки $A_i$ (вдоль оси).|
|$l_{3,i}$|float(мм)|Горизонтальное смещение точки $A_i$ стрелы относительно точки $G_{i-1}$ (относительно ГСК для первой срелы) в соответствии с рисунком. |
|$l_{4,i}$|float(мм)|Вертикальное смещение точки $A_i$ стрелы относительно точки $G_{i-1}$ (относительно ГСК для первой срелы) в соответствии с рисунком. |
|$j$|int|Номер блока|
|$K_{lFx(i,j)}$|str|Признак, показывающий от какой точки брать координату: от конца или начала стрелы (пока не используется)|
|$l_{F_x(i,j)}$|float (мм)|Перпендикулярное расстояние от блока до оси стрелы| 
|$l_{F_y(i,j)}$|float (мм)|Продольное расстояние (вдоль стрелы) от выбранной точки $K_{lFx}$ - (D или G) до проекции точки на ось стрелы |
|$D_i$|float(mm)|Диаметр блока|
|$Scheme_i$|int|Номер схемы|
|$Feature_{block}$|str|Расположение блока(на стреле, вне стрелы, на подвеске)|


## Выходные данные

| Переменная     | Тип / Формат          | Описание                     |
| -----------    | --------------------- | ---------------------------- |
|$X_{block(i,j)}$|float(мм)              |Координата X i-го блока|
|$Y_{block(i,j)}$|float(мм)              |Координата Y i-го блока|


Пример реализации в коде `Python`
-

![Alt text to image](/assets/algorithm/python1.png)

```python
import matplotlib.pyplot as plt
import math
import logging 
import matplotlib.patches as patches
from dataclasses import dataclass

# plt.set_loglevel(level="DEBUG")
# logging.getLogger('PIL.PngImagePlugin').setLevel(level="info")
logging.basicConfig(level = logging.DEBUG, force = True)

@dataclass
class BlockBindFixed:
    """Блок вне стрелы, барабан"""
    pass
@dataclass
class BlockBindBoom:
    """Блок на стреле"""
    boom: int
    def __init__(self, boom: int):
        self.boom = boom
@dataclass
class BlockBindHook:
    """Блок на подвеске"""
    pass
BlockBind = BlockBindFixed | BlockBindBoom | BlockBindHook

class Offset:
    x: float
    y: float
    def __init__(self, x: float, y: float):
            self.x = x
            self.y = y
    def __str__(self):
        return f'{self.x, self.y}'

class Boom:
    # Углы наклона стрел (относительно предыдыдущей) в градусах
    alpha_rel: float
    # Углы наклона стрел (относительно ГСК) в градусах
    alpha: float
    len: float
    l1: float
    l2: float
    l3: float
    l4: float
    D: Offset
    G: Offset
    def __init__(self, alpha_rel: float, len: float, l1: float, l2: float, l3: float, l4: float):
        """
        :alpha: Относительный угол наклона стрел (относительно предыдыдущей) в градусах
        :len: Длины стрел, мм
        :l1: Вертикальное смещение точки D, мм
        :l2: Горизонтальное смещение точки D, мм
        :l3: Вертикальное смещение начала стрелы относительно..., мм
        :l4: Горизонтальное смещение начала стрелы относительно..., мм
        """
        self.alpha_rel = alpha_rel
        self.alpha = 0.0
        self.len = len
        self.l1 = l1
        self.l2 = l2
        self.l3 = l3
        self.l4 = l4

class Block:
    lF: Offset
    D: float
    scheme: int
    bind: BlockBind
    coord: Offset
    def __init__(self, lF: Offset, D: float, scheme: int, bind: BlockBind):
        """
        :lF: Растояние от **конца** стрелы до оси блока, мм
        :D: Диаметры блоков, мм
        :schemes: Схема схода каната на блоке
        :boom: К какой стреле относится блок (нумерация с 0)
        """
        self.lF = lF
        self.D = D
        self.scheme = scheme
        self.bind = bind
        self.coord = Offset(0.0, 0.0)

# ---------------------------
# Вспомогательные функции
# ---------------------------

def XY_rotate(lx, ly, alpha):
    angle_rad = math.radians(alpha)
    x = lx * math.cos(angle_rad) - ly * math.sin(angle_rad)
    y = lx * math.sin(angle_rad) + ly * math.cos(angle_rad)
    return x, y

def alpha_horiz(Y1, Y2, X1, X2):
    """Угол наклона прямой к горизонту (в градусах)"""
    # Длина отрезка
    length = math.sqrt((X2 - X1)**2 + (Y2 - Y1)**2)
    if length == 0:
        return 0
    a = math.degrees(math.asin((Y1 - Y2) / length))
    if X1 <= X2:
        return a
    else:
        return 180 - a

def l_section(Y1, Y2, X1, X2):
    return math.sqrt((X2 - X1)**2 + (Y2 - Y1)**2)

def distance_point_to_line(Y1, Y2, X1, X2, x, y):
    """Расстояние от точки (x, y) до прямой, проходящей через (X1, Y1) и (X2, Y2)"""
    numerator = abs((Y2 - Y1) * x - (Y2 - Y1) * X1 - (X2 - X1) * y + (X2 - X1) * Y1)
    denominator = math.sqrt((Y2 - Y1)**2 + (X2 - X1)**2)
    if denominator == 0:
        return 0
    return numerator / denominator

def rope_parameters(X1, Y1, X2, Y2, D1, D2, k, j):
    l_block = l_section(Y1, Y2, X1, X2)
    alpha_block = alpha_horiz(Y1, Y2, X1, X2)
    alpha_rope = alpha_block + j * math.degrees(math.asin(0.5 * (D1 + k * D2) / l_block))
    X1_block = X1 + j * 0.5 * D1 * math.sin(math.radians(alpha_rope))
    Y1_block = Y1 + j * 0.5 * D1 * math.cos(math.radians(alpha_rope))
    X2_block = X2 - j * k * 0.5 * D2 * math.sin(math.radians(alpha_rope))
    Y2_block = Y2 - j * k * 0.5 * D2 * math.cos(math.radians(alpha_rope))
    
    l_rope = l_section(Y2_block, Y1_block, X2_block, X1_block)
    return {
        "l_block": l_block,
        "alpha_block": alpha_block,
        "alpha_rope": alpha_rope,
        "X1_block": X1_block,
        "Y1_block": Y1_block,
        "X2_block": X2_block,
        "Y2_block": Y2_block,
        "l_rope": l_rope
    }

# ------------------------------------------------
# Алгоритм расчета входа и исхода каната с блоков
# ------------------------------------------------
if __name__ == "__main__":
    # ---------------------------
    # Исходные данные
    # ---------------------------

    #
    # Стрелы
    booms = [
        Boom(alpha_rel= 69.71, len=11200.0, l1=0.0, l2=0.0, l3=0.0, l4=10330.0),
        Boom(alpha_rel= 155.3, len= 7984.0, l1=0.0, l2=0.0, l3=0.0, l4=    0.0),
    ]
    #
    # Блоки
    blocks = [
        Block(lF=Offset( 1830.0,  710.0), D=844.0, scheme=1, bind=BlockBindFixed()),
        Block(lF=Offset(  308.0, 1100.0), D=816.0, scheme=1, bind=BlockBindBoom(0)),
        Block(lF=Offset(-6550.0, 1730.0), D=816.0, scheme=1, bind=BlockBindBoom(1)),
        Block(lF=Offset(-1121.0,  973.0), D=816.0, scheme=2, bind=BlockBindBoom(1)),
        Block(lF=Offset(  267.0,  860.0), D=816.0, scheme=3, bind=BlockBindBoom(1)),
        Block(lF=Offset(  136.0,  -35.0), D=816.0, scheme=1, bind=BlockBindBoom(1)),
        Block(lF=Offset(    0.0,    0.0), D=  0.0, scheme=0, bind=BlockBindHook()),
    ]

    block_bind = [
        BlockBindFixed(),   # Блок 1
        BlockBindBoom(1),   # Блок 2
        BlockBindBoom(2),   # Блок 3
        BlockBindBoom(2),   # Блок 4
        BlockBindBoom(2),   # Блок 5
        BlockBindBoom(2),   # Блок 6
        BlockBindHook(),    # Блок 7
    ]

    
    # Дополнительные параметры крюковой подвески
    hook_params = {
        "hook_block_num": 7,  # номер блока подвеса в списке
        "l_hook": 1000        # длина подвеса (мм)
    }

    # Параметры для расчета длины каната
    rope_calc_params = {
        "D_pitch": 845,  # делительный диаметр барабана (мм)
        "a_n": 1,        # количество канатов на барабане
        "Lstock": 0,     # дополнительный запас каната (мм)
        "Lfact": 88000      # фактическая длина каната (мм)
    }

    # ---------------------------
    # 2. Угол наклона к горизонту каждой стрелы (alpha_boom)
    # ---------------------------
    alpha_sum = 0.0
    for i, boom in enumerate(booms):
        alpha_sum += boom.alpha_rel
        # log.debug(f"i: {i},  alpha sum_ {alpha_sum}")
        boom.alpha = alpha_sum - i * 180

    # ---------------------------
    # 3. Матрица T (D и G для каждой стрелы)
    # ---------------------------
    for i, boom in enumerate(booms):
        # Начало стрелы
        if i == 0:
            x0, y0 = 0, 0
            alpha_prime = 90
        else:
            x0, y0 = booms[i - 1].G.x, booms[i - 1].G.y
            alpha_prime = booms[i - 1].alpha

        wx, wy = XY_rotate(boom.l4, boom.l3, alpha_prime)
        XY_start = Offset(x0 + wx, y0 + wy)
        # log.debug(f"Стрела {i}: XY_start={XY_start}")

        # Точка D
        Dx, Dy = XY_rotate(- boom.l2, boom.l1, boom.alpha)
        D_point = Offset(XY_start.x + Dx, XY_start.y + Dy)
        # log.debug(f"\t D_point={D_point}")

        # Точка G
        Gx, Gy = XY_rotate(boom.len - boom.l2, boom.l1, boom.alpha)
        G_point = Offset(XY_start.x + Gx, XY_start.y + Gy)
        # log.debug(f"\t G_point={G_point}")

        boom.D = D_point
        boom.G = G_point
    logging.debug(f"booms {booms}")

    # ---------------------------
    # 4. Координаты блоков XY_block
    # ---------------------------
    for idx, block in enumerate(blocks):
        # bind = block_bind[idx]
        match block.bind:
            case BlockBindFixed():
                # Формула из алгоритма:
                dx1, dy1 = XY_rotate(-block.lF.x, block.lF.y, 0)
                dx2, dy2 = XY_rotate(booms[0].l4, booms[0].l3, 90)  # от первой стрелы
                x = dx1 + dx2
                y = dy1 + dy2
                block.coord.x = x
                block.coord.y = y
            case BlockBindBoom(boom_index):
                # Определяем номер стрелы
                # boom_num = int(feature.split()[0]) - 1
                boom = booms[boom_index]
                # logging.debug(f"Стрела {idx}")
                base_point = boom.G  # точка G
                dx, dy = XY_rotate(block.lF.x, block.lF.y, boom.alpha)
                block.coord.x = base_point.x + dx
                block.coord.y = base_point.y + dy
            case BlockBindHook():
                block.coord.x = float('nan')
                block.coord.y = float('nan')
            case _:
                raise ValueError(f"Неизвестный тип блока [{idx}]: {bind}")
                

    # ---------------------------
    # 5. Расчёт координат крюковой подвески
    # ---------------------------
    hook_block_num = hook_params["hook_block_num"]
    l_hook = hook_params["l_hook"]
    
    prev_idx = hook_block_num - 2
    x_prev, y_prev = blocks[prev_idx].coord.x, blocks[prev_idx].coord.y
    D_prev = blocks[prev_idx].D
    
    blocks[hook_block_num - 1].coord.x = x_prev + 0.5 * D_prev
    blocks[hook_block_num - 1].coord.y = y_prev - l_hook

    logging.debug(f"Крюковая подвеска: ({blocks[hook_block_num - 1].coord.x}, "
                  f"{blocks[hook_block_num - 1].coord.y})")



    # ---------------------------
    # 6. Расчёт параметров каната
    # ---------------------------
    rope_data = []
    for i, block in enumerate(blocks[:-1]):
        X1, Y1 = block.coord.x, block.coord.y
        X2, Y2 = blocks[i + 1].coord.x, blocks[i + 1].coord.y
        D1 = block.D
        D2 = blocks[i + 1].D
        scheme = block.scheme

        if scheme == 1: k, j = -1, 1
        elif scheme == 2: k, j = 1, 1
        elif scheme == 3: k, j = 1, -1
        elif scheme == 4: k, j = -1, -1
        else: raise ValueError(f"Некорректная схема: {scheme}")

        params = rope_parameters(X1, Y1, X2, Y2, D1, D2, k, j)
        params["block_pair"] = (i + 1, i + 2)
        params["scheme"] = scheme
        rope_data.append(params)
   
    logging.debug("Параметры каната между блоками (rope_data):")
    for r in rope_data:
       logging.debug(f" Пары {r['block_pair']}: l_block={r['l_block']:.2f}, alpha_rope={r['alpha_rope']:.2f}, l_rope={r['l_rope']:.2f}")

    # -----------------------------
    # 7. Углы обхвата и длины дуг каждого блока
    # -----------------------------


    def calc_block_angles_and_arcs(rope_data):
        wrap_angles = []
        arc_lengths = []
        L_sys_arc = 0
    
        prev_alpha = None  # предыдущий угол для формирования alpha_rope_list
    
        for block, r in zip(blocks[:-1], rope_data):
            alpha_rope = r.get("alpha_rope", 0)

            # Формируем alpha_rope_list
            if prev_alpha is not None:
                alpha_rope_list = [prev_alpha, alpha_rope]
            else:
                alpha_rope_list = [alpha_rope]  # для первого блока
            r["alpha_rope_list"] = alpha_rope_list
        
            # Расчёт угла обхвата
            if len(alpha_rope_list) > 1:
                alpha_wrap = abs(alpha_rope_list[-1] - alpha_rope_list[0])
            else:
                alpha_wrap = 0
            
            R = block.D / 2
            L_arc = (math.pi * R * alpha_wrap) / 180
            
            L_sys_arc += L_arc
            wrap_angles.append(alpha_wrap)
            arc_lengths.append(L_arc)

            prev_alpha = alpha_rope  # обновляем предыдущий угол
    
        return {
            "wrap_angles": wrap_angles,
            "arc_lengths": arc_lengths,
            "L_sys_arc": L_sys_arc
        }

    # -----------------------------
    # 8. Общая длина каната и остаток на барабане
    # -----------------------------
    def calc_rope_sums(rope_data, D_pitch, a_n, Lstock, Lfact):
        l_section_summ = sum(r["l_rope"] for r in rope_data)
        L_stock_total = 3 * math.pi * D_pitch * a_n + Lstock
        block_results = calc_block_angles_and_arcs(rope_data)
        L_rope_sum = l_section_summ + block_results["L_sys_arc"] + L_stock_total
        L_winch = Lfact - l_section_summ - block_results["L_sys_arc"]

    
        return {
            "l_section_summ": l_section_summ,
            "L_stock_total": L_stock_total,
            "L_rope_sum": L_rope_sum,
            "L_winch": L_winch,
            "block_results": block_results
        }
        
# ---------------------------
# Логи
# ---------------------------
logging.debug(f"Число стрел: {len(booms)}")
logging.debug(f"alpha_boom: {[round(boom.alpha, 3) for boom in booms]}")
for idx, boom in enumerate(booms, start=1):
    logging.debug(f"Стрела {idx}: D={boom.D}, G={boom.G}")
for i, block in enumerate(blocks, start=1):
    logging.debug(f"Блок {i}: {block.coord}")
for r in rope_data:
    logging.debug(f"Блоки {r['block_pair']} | Схема {r['scheme']} | "
                  f"L_block={r['l_block']:.2f} | Alpha_rope={r['alpha_rope']:.2f}° | "
                  f"L_rope={r['l_rope']:.2f}")

block_results = calc_block_angles_and_arcs(rope_data)
logging.debug("Углы обхвата и длины дуг (по каждой паре блоков):")

for i, (alpha_wrap, arc_length) in enumerate(
        zip(block_results["wrap_angles"], block_results["arc_lengths"]), start=1):
    bp = f"{rope_data[i-1]['block_pair'][0]}-{rope_data[i-1]['block_pair'][1]}"
    logging.debug(
        f"Блок {bp}: alpha_wrap={alpha_wrap:.2f} deg, arc_length={arc_length:.2f} mm"
    )

rope_results = calc_rope_sums(
    rope_data,
    D_pitch=rope_calc_params["D_pitch"],
    a_n=rope_calc_params["a_n"],
    Lstock=rope_calc_params["Lstock"],
    Lfact=rope_calc_params["Lfact"]
)



logging.debug("---- Итоги расчёта каната ----")
logging.debug(f"Сумма прямых участков l_section_summ = {rope_results['l_section_summ']:.2f} мм")
logging.debug(f"Сумма дуг L_sys_arc                 = {rope_results['block_results']['L_sys_arc']:.2f} мм")
logging.debug(f"Запас на барабане L_stock_total     = {rope_results['L_stock_total']:.2f} мм")
logging.debug(f"Общая требуемая длина L_rope_sum    = {rope_results['L_rope_sum']:.2f} мм")
logging.debug(f"Остаток на барабане L_winch        = {rope_results['L_winch']:.2f} мм")


    # # ---------------------------
    # # Построение графика
    # # ---------------------------
    # plt.figure(figsize=(10, 8))
    # plt.title("Схема расположения стрел и блоков")
    # plt.xlabel("X координата (мм)")
    # plt.ylabel("Y координата (мм)")
    # plt.grid(True)
    # plt.axis('equal')

    # # Стрелы
    # colors = ['green', 'blue']
    # for i, boom in enumerate(booms):
    #     plt.plot([boom.D.x, boom.G.x], [boom.D.y, boom.G.y], color=colors[i], linewidth=1.5)
    #     plt.scatter([boom.D.x, boom.G.x], [boom.D.y, boom.G.y], color=colors[i], s=20, marker='s')

    # # Блоки
    # for i, block in enumerate(blocks):
    #     x, y = block.coord.x, block.coord.y
    #     if math.isnan(x) or math.isnan(y):
    #         continue
    #     radius = block.D / 2
    #     circle = patches.Circle((x, y), radius, fill=False, color='deepskyblue', linewidth=1)
    #     plt.gca().add_patch(circle)
    #     plt.text(x + radius, y + radius, f'Блок {i+1}', fontsize=8, color='black')

    # # Канаты
    # for r in rope_data:
    #     plt.plot([r["X1_block"], r["X2_block"]], [r["Y1_block"], r["Y2_block"]],
    #             color='blue', linestyle='--')
    #     plt.scatter([r["X1_block"], r["X2_block"]], [r["Y1_block"], r["Y2_block"]],
    #                 color='orange', s=15)

    # plt.show()
```
