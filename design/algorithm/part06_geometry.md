# Расчет координат блоков на кране

**Цель** - рассчитать глобальные координаты центров блоков, установленных на стреле крана. Расчет основан на длине и угле наклона каждой стрелы, а также на локальных координатах блоков относительно начала стрел.

Схема крана включает в себя:

- **Две стрелы:**
  - `main boom` — основная стрела, длина обозначается: $L_{boom(1)}$
  - `knuckle boom` — складная стрела, длина обозначается $L_{boom(2)}$
- **Шесть блоков:**
  - 1 блок — барабан лебедки, который стоит отдельно от стрел и его координаты будут определяться отдельно
  - 2-6 блоки — блоки на стрелах
  - 7 блок - блок на стреле

![Alt text to image](/assets/algorithm/Crane1.png)


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

Для определения параметров каната между блоками используется функция `rope_parameters` на вход которой необходимы следующие параметры:

```python
rope_parameters(X1, Y1, X2, Y2, D1, D2, k, j):
```
Далее рассмотрим алгоритм определения параметров в этой функции.


**1. Определение схемы (`scheme`) схода каната** --> присваивание значений $k$ и $j$ в соответсвии с рисунком и таблицей ниже. 

![Alt text to image](/assets/algorithm/схема_схода_каната.jpg)

Существуют 4 схемы схода каната, в соответсвие с рисунком необходимо определить к какому типу схемы относится сход каната на i-том блоке. Далее по нему присваиваются значения $k$ и $j$ , как представлено ниже.

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
В проекте для крана ЯСЗ, крюковая подвеска выполняется без блока, поэтому на данном этапе рассматривается конфигурация крюковой подвески без блока.

Расчет крюковой подвески выполняется по следующей формуле
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

## 6.1 Определение координат крюковой подвески (КП) (опционально*)
Положение крюковой подвески зависит от множества факторов, включая приложенные к грузу силы, угол крена судна, длину подвеса, степень опускания крюка и другие параметры.
Также крюковая подвеска выполняется в разной конфигурации: без блока, с одним блоком, с двумя блоками.

Определение координат при наличии блоков на КП, выполняются в другом виде, расчет которых также будет добавлен позднее.

## 7. Расчет параметров каната

Полную длину каната можно определить двумя способами:

1. Принять как исходную величину, если она известна.
2. Рассчитать. Для этого нужно:
    - Найти «особое» положение крана, при котором канатоемкость лебедки максимальна, а длина каната на крюковой подвеске минимальна (1200 мм).
    - В этом положении просуммировать:
       - Длины всех прямолинейных участков каната.
       - Длины всех дуг
       - Длину каната на лебедке ($L_{winch}$)

На данном этапе «особое» положение крана задаётся исходными данными — двумя углами, соответствующими максимальному вылету. Далее будет определяться алгоритмом данное положение.

Для расчёта каната между блоками используется функция `rope_parameters`, которая принимает координаты центров двух блоков, их диаметры и схему схода каната(k, j).

Для определения каната между блоками, используется функция 
```python
rope_parameters(X1, Y1, X2, Y2, D1, D2, k, j)
```
Данный этап нужен для того, чтобы вычислить фактическую суммарную длину каната, проходящего:
- по прямолинейным участкам между блоками,
- по дугам, обхватывающим блоки,
- а также учесть каната на барабане лебёдки.
  
Алгоритм состоит из следующих этапов:

### 7.1 Длина прямолинейного участка

- Для каждого блока рассчитывается длина прямого участка каната $l_{section(i)}$ — это длина отрезка между двумя точками схода каната.

- Далее прямолинейные участки суммируются

$$l_{section_{summ}} = \sum l_{section(i)}$$


### 7.2 Рассчитывается длина дуги на блоках

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

![Alt text to image](/assets/algorithm/wrap.png)

**2. Далее определить длину дуги на блоке**

Длина дуги на блоке определяется по углу обхвата , посчитанному выше и по радиусу блока 

$$L_{arc} = \frac{\pi \cdot R \cdot \alpha_{wrap}}{180}$$
где
- $R_j = \frac {D_j}{2}$

и по этой формуле собираем массив $L_{sys_{arc}}$, где есть длина каната на каждом блоке

### 7.3 Определение длины каната на барабане (пока не учитываем)

Для расчета нам необходим:
- Делительный диаметр барабана (диаметр по центрам каната 1 слоя барабана)*
$$D_1 = 845 мм$$
- Количество канатов сходящих с барабана*
$$a_n = 1$$
- Дополнительный запас каната на барабане сверх необходимых минимальных 3х витков*
$$L_{stock} = 0 м$$
- Фактическая суммарная длина каната
$$L_{stock(total)} = 3 \cdot \pi \cdot D_1 \cdot a_n + L_{stock}$$

*пункты под звездочкой пока не учитываются в расчете

### 7.4 Определение полной длины каната
В "особом" положении крана, при котором канатоемкость лебедки максимальная (исходная данная) и при этом длина каната на крюковой подвеске минимальная, то есть 1200мм, при таком положении просуммировать:
   - длины прямолинейных участков
   - длины дуг
   - длина каната на лебедке

$$L_{fact} = l_{section_{summ}} + L_{sys_{arc}} + L_{winch}$$

### 8. Длина каната на лебедке и положение крюковой подвески с учетом вытравливания каната
Поскольку положение крана изменяется, конфигурация каната также меняется. Это приводит к изменению суммарной длины прямолинейных участков и дуг, что, в свою очередь, влияет на длину каната на лебедке и длину крюковой подвески.

Алгоритм переопределения крюковой подвески зависит от нескольких факторов:
Существуют ограничения по минимальной длине подвеса, так как 
- **При подьеме хобота** сумма длин прямолинейных участков и сумма дуг уменьшается => часть каната, находящегося на блоках и между ними, высвобождается, тем самым увеличивая высоту подвеса груза.
- **При опускании хобота** высота подвеса пытается сократиться, но она не может быть меньше 1200мм, датчик ограничивающий минимальную высоту подвеса блокирует данное движение, для возобновления движения необходимо часть каната стравить с лебёдки для обеспечения минимальной высоты подвеса l_hook_min. 

Зная канатоемкость на лебедке L_winch и общую длину каната мы можем вычислить баланс длины **x** следующим образом : 
```python
x = Lfact - L_winch - l_section_summ - L_sys_arc  
```
- При x < 0 необходимо вытравить каната, так как его не хватает для обеспечения миниимальной длины подвеса 
- При x > 0 есть избыток, удлиняем подвес на x, барабан не трогаем
- При x = 0, оставляем все как есть 

![Alt text to image](/assets/algorithm/rope_var.png)

### 9. Алгоритм определения опорных точек

Для определения точек перегиба на кране необходимо определить опорные точки по длине каната крана. Для этого создается функция *build_support_points*, которая определяет точки входа и выхода каната на блок и с блока, где канат испытывает изгиб. Данная функция поможет определить когда определенная точка каната входит или выходит в зону изгиба, тем самым накапливая износ.

Алгоритм работы (пошагово):

**1) Исходная точка (Крюк):** Расчет начинается с позиции крюка. За начальную точку (F14) принимается полная длина каната Lfact (в миллиметрах), это величина неизменна.

**2) Обратный порядок данных**: Данные о длинах прямых участков троса (rope_data.l_rope) и длинах дуг на блоках (block_results.arc_lengths) изначально хранятся в порядке от барабана к крюку. Для удобства расчета "от крюка к барабану" эти массивы переворачиваются (реверсируются).

**3) Последовательное вычитание:** Алгоритм движется от крюка назад к барабану, последовательно вычитая из общей длины Lfact длину каждого пройденного участка:
- Сначала вычитается длина прямого участка троса, ведущего к предыдущему блоку.
- Затем вычитается длина дуги огибания этого блока.

Этот процесс повторяется для каждой пары "прямой участок -> дуга блока" до тех пор, пока не будет достигнута начальная точка на барабане.

**4) Формирование массива точек:** Каждый раз после вычитания текущее значение L_total записывается в массив как новая опорная точка. В итоге получается массив точек F, описывающий путь в направлении от крюка к барабану.


### 10. Условие переваливания каната при опускании хобота
Если угол между канатом на предпоследнем блоке становится меньше 90°, этот блок исключается из расчётов. В этом случае канат считается идущим напрямую с предыдущего блока на следующий (например, в случае 7 блоков, блок 6 исключается, и рассчитывается прямой участок между блоками 5 и 7).

![Alt text to image](/assets/algorithm/переваливание_каната.png)

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


``` python
import matplotlib.pyplot as plt
import math
import logging
import numpy as np
import matplotlib.patches as patches
from dataclasses import dataclass

logging.getLogger("PIL").setLevel(logging.WARNING)
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
    """"Формула расчета координат в повернутой СК"""
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
    """"Расчет расстояния между двумя точками"""
    return math.sqrt((X2 - X1)**2 + (Y2 - Y1)**2)

def distance_point_to_line(Y1, Y2, X1, X2, x, y):
    """Расстояние от точки (x, y) до прямой, проходящей через (X1, Y1) и (X2, Y2)"""
    numerator = abs((Y2 - Y1) * x - (Y2 - Y1) * X1 - (X2 - X1) * y + (X2 - X1) * Y1)
    denominator = math.sqrt((Y2 - Y1)**2 + (X2 - X1)**2)
    if denominator == 0:
        return 0
    return numerator / denominator

def rope_parameters(X1, Y1, X2, Y2, D1, D2, k, j):
    """
    Расчет параметров каната
    l_block - расстояние между блоками, на картинке L_block
    alpha_block - угол между линией между блоками и горизонтом на картинке alpha_block
    alpha_rope - угол между линией каната между блоками и горизонтом на картинке alpha_rope
    X1_block - точка входа на блок по X
    Y1_block - точка входа на блок по Y
    X2_block - точка выхода на блок по X
    Y2_block - точка выхода на блок по Y
    """
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
    # Данные стрел и блоков
    # booms = [
    #     Boom(alpha_rel= 10.4538273, len=11200.0, l1=0.0, l2=0.0, l3=0.0, l4=10330.0),
    #     Boom(alpha_rel= 155.3, len= 7984.0, l1=0.0, l2=0.0, l3=0.0, l4=    0.0),
    # ]
    booms = [
        Boom(alpha_rel= 65, len=11200.0, l1=0.0, l2=0.0, l3=0.0, l4=10330.0),
        Boom(alpha_rel= 125, len= 7984.0, l1=0.0, l2=0.0, l3=0.0, l4=    0.0),
    ]
    blocks = [
        Block(lF=Offset( 1830.0,  710.0), D=845.000, scheme=1, bind=BlockBindFixed()),
        Block(lF=Offset(  308.0, 1100.0), D=816.000, scheme=1, bind=BlockBindBoom(0)),
        Block(lF=Offset(-6549.0, 1730.0), D=816.000, scheme=1, bind=BlockBindBoom(1)),
        Block(lF=Offset(-1121.0,  973.0), D=816.000, scheme=2, bind=BlockBindBoom(1)),
        Block(lF=Offset(  267.0,  860.0), D=816.000, scheme=3, bind=BlockBindBoom(1)),
        Block(lF=Offset(  136.0,  -35.0), D=816.000, scheme=1, bind=BlockBindBoom(1)),
        Block(lF=Offset(    0.0,    0.0), D=    0.0, scheme=0, bind=BlockBindHook()),
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
    """
    Lfact - фактическая длина каната
    L_winch - длина каната на лебедке в основном положении
    lhook_min - минимальная длина подвеса
    hook_block_num - номер блока крюковой подвески
    """ 
    rope_calc_params = {"Lfact": 82243, "L_winch": 58330, "lhook_min": 1200, "hook_block_num": 7}

    # ---------------------------
    # 1. Угол наклона к горизонту каждой стрелы (alpha_boom)
    # ---------------------------
    alpha_sum = 0.0
    for i, boom in enumerate(booms):
        alpha_sum += boom.alpha_rel
        # log.debug(f"i: {i},  alpha sum_ {alpha_sum}")
        boom.alpha = alpha_sum - i * 180

    # ---------------------------
    # 2. Матрица T (D и G для каждой стрелы)
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

        # Точка D
        Dx, Dy = XY_rotate(- boom.l2, boom.l1, boom.alpha)
        D_point = Offset(XY_start.x + Dx, XY_start.y + Dy)

        # Точка G
        Gx, Gy = XY_rotate(boom.len - boom.l2, boom.l1, boom.alpha)
        G_point = Offset(XY_start.x + Gx, XY_start.y + Gy)

        boom.D = D_point
        boom.G = G_point

    # ---------------------------
    # 3. Координаты блоков XY_block
    # ---------------------------
    for idx, block in enumerate(blocks):
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
                boom = booms[boom_index]
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
    # 4. Расчёт координат крюковой подвески
    # ---------------------------
    hook_block_num = rope_calc_params["hook_block_num"]
    l_hook = rope_calc_params["lhook_min"]
    
    prev_idx = hook_block_num - 2
    x_prev, y_prev = blocks[prev_idx].coord.x, blocks[prev_idx].coord.y
    D_prev = blocks[prev_idx].D
    
    blocks[hook_block_num - 1].coord.x = x_prev + 0.5 * D_prev
    blocks[hook_block_num - 1].coord.y = y_prev - l_hook

    # ---------------------------
    # 5. Расчёт параметров каната
    # ---------------------------

    rope_data = []
    for i, block in enumerate(blocks[:-1]):
        X1, Y1 = block.coord.x, block.coord.y
        X2, Y2 = blocks[i + 1].coord.x, blocks[i + 1].coord.y
        D1 = block.D
        D2 = blocks[i + 1].D
        scheme = block.scheme
    
        if scheme == 1:
            k, j = -1, 1
        elif scheme == 2:
            k, j = 1, 1
        elif scheme == 3:
            k, j = 1, -1
        elif scheme == 4:
            k, j = -1, -1
        else:
            raise ValueError(f"Некорректная схема: {scheme}")
    
        params = rope_parameters(X1, Y1, X2, Y2, D1, D2, k, j)
        params["block_pair"] = (i + 1, i + 2)   # 1-базовая нумерация блоков
        params["scheme"] = scheme
        rope_data.append(params)      
    """
    Далее будет задаваться дополнительное условие, 
    которое учитывает переваливание каната при положении крана хоботом вниз
    """
    alpha_pen = rope_data[-2].get("alpha_rope")  # градусы
    #  если предпоследний < 90°, выкидываем блок №6 (1-based) и шьём 5-7 блоки
    if alpha_pen > 90.0:
        # находим позиции сегментов (5→6) и (6→7)
        idx_56 = next((idx for idx, r in enumerate(rope_data) if r.get("block_pair") == (5, 6)), None)
        idx_67 = next((idx for idx, r in enumerate(rope_data) if r.get("block_pair") == (6, 7)), None)

        lhook_min = float(rope_calc_params.get("lhook_min"))
        l_hook = float(rope_data[-1].get("l_rope"))

        b5 = blocks[4]  # 0-based: блок 5
        b7 = blocks[6]  # 0-based: блок 7 (КП)
        
        # определение координат кп 
        b7.coord.x = b5.coord.x - 0.5 * b5.D
        b7.coord.y = b5.coord.y - l_hook
        # вычислим новый сегмент 5-7
        X1, Y1 = b5.coord.x, b5.coord.y
        X2, Y2 = b7.coord.x, b7.coord.y
        D1, D2 = b5.D, b7.D
        scheme = getattr(b5, "scheme", 1)

        if scheme == 1:
            k, j = -1, 1
        elif scheme == 2:
            k, j = 1, 1
        elif scheme == 3:
            k, j = 1, -1
        elif scheme == 4:
            k, j = -1, -1
        else:
            raise ValueError(f"Некорректная схема: {scheme}")

        new_params = rope_parameters(X1, Y1, X2, Y2, D1, D2, k, j)
        new_params["block_pair"] = (5, 7)
        new_params["scheme"] = scheme

        # удаляем (5-6) и (6-7), вставляем (5-7) на место прежнего (5-6)
        for idx in sorted([idx_56, idx_67], reverse=True):
            rope_data.pop(idx)
        rope_data.insert(idx_56, new_params)
            
    # -----------------------------
    # 6. Углы обхвата и длины дуг каждого блока
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
    # 7. Общая длина каната, сумма длин прямолинейных участков и сумма длин дуг
    # -----------------------------
    def calc_rope_sums(rope_data, Lfact):
        l_section_summ = sum(r["l_rope"] for r in rope_data)
        block_results = calc_block_angles_and_arcs(rope_data)
        return {
            "l_section_summ": l_section_summ,
            "block_results": block_results
        }
    rope_results = calc_rope_sums(
        rope_data,
        Lfact=rope_calc_params["Lfact"]
    )

    # -----------------------------.
    # 8. Построение опорных точек от крюка к барабану
    # -----------------------------    
    def build_support_points(rope_results, rope_data, block_results, Lfact):
        """
        Формируем 14 опорных точек (в метрах) от крюка к барабану:
          F14 = Lfact (крюк)
          F13 = F14 - l_rope_7
          F12 = F13 - arc_6
          F11 = F12 - l_rope_6
          ...
          F1  = F2 - arc_1
        """
        L_total = Lfact  # мм, начинаем с полной длины (крюк)
        # Получаем длины участков в порядке (от крюка к барабану)
        l_sections = [r["l_rope"] for r in reversed(rope_data)]   # мм
        arcs = list(reversed(block_results["arc_lengths"]))       # мм
        F = [L_total]  # F12 (крюк)

        # Формируем остальные точки
        for i in range(len(l_sections)):

            # Вычитаем прямой участок
            if i < len(l_sections):
                L_total -= l_sections[i]
                F.append(L_total)
                
            # Вычитаем дугу
            if i < len(arcs):
                if arcs[i] > 0:
                    L_total -= arcs[i]
                    F.append(L_total)
                else:
                    continue
            else:
                logging.debug(f"Дуга для участка {i+1} не существует")
            
        # Переворачиваем, чтобы получить порядок от барабана к крюку
        F = list(reversed(F))
        F = np.array(F, dtype=float) / 1000.0  # метры
        return F


    def ensure_min_hook_length(booms, blocks, rope_calc_params):
        """
        Гарантирует минимальную длину подвеса (lhook_min).
        Если при текущей геометрии и lhook_min суммарной длины каната не хватает,
        считает, сколько нужно вытравить с барабана (уменьшить L_winch).
    
        Возвращает кортеж:
          (need_payout, new_L_winch, x)
    
        Где:
          need_payout  > 0  -> столько мм надо вытравить с лебёдки
          need_payout == 0  -> ничего делать не надо
          new_L_winch       -> предложенное новое значение L_winch
          x                 -> баланс длины: Lfact - L_winch - (прямые + дуги)
                                  (если x < 0, длины не хватает)
        """

        # Суммы прямых и дуг 
        block_results = calc_block_angles_and_arcs(rope_data)
        l_section_summ = sum(r["l_rope"] for r in rope_data)
        L_sys_arc = block_results["L_sys_arc"]
    
        # Баланс длины (мм): сколько осталось после барабана
        Lfact = rope_calc_params["Lfact"]
        L_winch = rope_calc_params["L_winch"]
        x = Lfact - L_winch - l_section_summ - L_sys_arc  # x<0: не хватает; x>0: есть избыток
        
        if x < 0:
            # Не хватает каната -> нужно вытравить с лебёдки
            need_payout = -x                           # сколько мм вытравить
            new_L_winch = L_winch - need_payout       # остаток на барабане станет меньше
    
        elif x > 0:
            # Есть избыток -> удлиняем подвес на x, барабан не трогаем
            need_payout = 0.0
            new_L_winch = L_winch
        
            # Найдём, какой блок перед крюком
            prev_idx = rope_data[-1].get("block_pair", (len(blocks)-1, len(blocks)))[0] - 1
            hook_idx = rope_data[-1].get("block_pair", (len(blocks)-1, len(blocks)))[1] - 1
        
            # Текущая минимальная длина подвеса, которую ставили изначально
            lhook_min = float(rope_calc_params.get("lhook_min", 1200.0))
        
            # Поставим крюковую подвеску строго под prev-блоком на lhook_min + x
            b_prev = blocks[prev_idx]
            b_hook = blocks[hook_idx]
        
            # используем тот же сдвиг по X, что и выше (x_prev + 0.5*D_prev)
            b_hook.coord.x = b_prev.coord.x + 0.5 * b_prev.D
            b_hook.coord.y = b_prev.coord.y - (lhook_min + x)
        
            # Пересчитаем последний участок (prev -> hook)
            X1, Y1 = b_prev.coord.x, b_prev.coord.y
            X2, Y2 = b_hook.coord.x, b_hook.coord.y
            D1, D2 = b_prev.D, b_hook.D
            scheme = getattr(b_prev, "scheme", 1)
        
            if   scheme == 1: k, j = -1,  1
            elif scheme == 2: k, j =  1,  1
            elif scheme == 3: k, j =  1, -1
            elif scheme == 4: k, j = -1, -1
            else:
                raise ValueError(f"Некорректная схема: {scheme}")
        
            last_seg = rope_parameters(X1, Y1, X2, Y2, D1, D2, k, j)
            last_seg["block_pair"] = (prev_idx + 1, hook_idx + 1)
            last_seg["scheme"] = scheme
            rope_data[-1] = last_seg  # заменить последний сегмент полностью
            
        else:
            # Точно в баланс
            need_payout = 0.0
            new_L_winch = L_winch
            
        return need_payout, new_L_winch, x , rope_data[-1]['l_rope']

#############################################################
# Расчет дуг и канатов
block_results = calc_block_angles_and_arcs(rope_data)
# Строим опорные точки
support_points = build_support_points(rope_results, rope_data, block_results, rope_calc_params["Lfact"])
# Запомним исходную длину последнего прямого участка (подвеса)
last_len_before = rope_calc_params["lhook_min"]
# Cчитаем количество каната которое надо вытравить 
need_payout, new_L_winch, x, rope_data[-1]['l_rope'] = ensure_min_hook_length(booms, blocks, rope_calc_params)

              
# ---------------------------
# Логи
# ---------------------------
logging.debug('-'*40)
logging.debug(f"Число стрел: {len(booms)}")
logging.debug('-'*40)
logging.debug(f"alpha_boom: {[round(boom.alpha, 3) for boom in booms]}")
logging.debug('-'*40)
for idx, boom in enumerate(booms, start=1):
    logging.debug(f"Стрела {idx}: D={boom.D}, G={boom.G}")
logging.debug('-'*40)
for i, block in enumerate(blocks, start=1):
    logging.debug(f"Блок {i}: Координаты блока {block.coord}")
logging.debug('-'*40)
for r in rope_data:
    logging.debug(f"Блоки {r['block_pair']} | Схема {r['scheme']} | "
                  f"L_block={r['l_block']:.2f} | Alpha_rope={r['alpha_rope']:.2f}° | "
                  f"L_rope={r['l_rope']:.2f}")
logging.debug('-'*40)
for r in rope_data:
    logging.debug(
        f'Вход X {r["X1_block"]:.2f}, Выход X {r["X2_block"]:.2f} | '
        f'Вход Y {r["Y1_block"]:.2f}, Выход Y {r["Y2_block"]:.2f}'
    )
logging.debug('-'*40)
logging.debug("Углы обхвата и длины дуг (по каждой паре блоков):")
for i, (alpha_wrap, arc_length) in enumerate(
        zip(block_results["wrap_angles"], block_results["arc_lengths"]), start=1):
    bp = f"{rope_data[i-1]['block_pair'][0]}-{rope_data[i-1]['block_pair'][1]}"
    logging.debug(
        f"Блок {bp}: угол обхвата = {alpha_wrap:.3f} deg, длина дуги = {arc_length:.3f} mm"
    )
logging.debug('-'*40)
logging.debug("Опорные точки")
for i, v in enumerate(support_points, start=1):
    logging.debug(f"F{i:02d}: {v:8.3f}")
logging.debug('-'*40)
logging.debug("Итоги расчёта каната")

balance_rope = (rope_calc_params['Lfact'] - rope_calc_params['L_winch'] - 
     rope_results['l_section_summ'] - rope_results['block_results']['L_sys_arc'])

l_rope_sum =  rope_calc_params['L_winch'] + rope_results['l_section_summ'] + rope_results['block_results']['L_sys_arc'] + balance_rope
            
logging.debug(f"Rope rest  = {balance_rope}")
logging.debug(f"Min hook   = {rope_calc_params['lhook_min']}")
logging.debug(f"l_rope_sum = {l_rope_sum}")
logging.debug(f"Lfact      = {rope_calc_params['Lfact'] }")
logging.debug(f"new_L_winch= {new_L_winch}")
logging.debug(f"L_winch    = {rope_calc_params['L_winch']}")
logging.debug(f"l_sec_sum  = {rope_results['l_section_summ']}")
logging.debug(f"L_sys_arc  = {rope_results['block_results']['L_sys_arc']}")
logging.debug(f"l_hook     = {rope_data[-1]['l_rope']:.2f}")

# ---------------------------
# Построение графиков
# ---------------------------

# 1 График крана, блоков, каната
plt.figure(figsize=(10, 8))
plt.title("Схема расположения стрел и блоков")
plt.xlabel("X координата (мм)")
plt.ylabel("Y координата (мм)")
plt.grid(True)
plt.axis('equal')

# Стрелы
colors = ['black', 'black']
for i, boom in enumerate(booms):
    plt.plot([boom.D.x, boom.G.x], [boom.D.y, boom.G.y], color=colors[i], linewidth=1.5)
    plt.scatter([boom.D.x, boom.G.x], [boom.D.y, boom.G.y], color=colors[i], s=20, marker='s')

# Блоки
for i, block in enumerate(blocks, start=1):
    x, y = block.coord.x, block.coord.y
    radius = block.D / 2
    circle = patches.Circle((x, y), radius, fill=False, color='deepskyblue', linewidth=1)
    plt.gca().add_patch(circle)
    plt.text(x, y, f'{i}', fontsize=8, color='black', 
             ha='center', va='center', weight='bold',
             bbox=dict(boxstyle="circle, pad=0.3", facecolor='white', edgecolor='black', alpha=0.7))
   
# Канаты
for r in rope_data:
    plt.plot([r["X1_block"], r["X2_block"]], [r["Y1_block"], r["Y2_block"]],
            color='blue', linestyle='--')
    plt.scatter([r["X1_block"], r["X2_block"]], [r["Y1_block"], r["Y2_block"]],
                color='orange', s=25)

# Изменения длины последнего участка (у КП)
last_len_after = rope_data[-1]['l_rope']
last_len_before = rope_calc_params['lhook_min']
delta_last = last_len_after - last_len_before
r_last = rope_data[-1]
x1, y1 = r_last["X1_block"], r_last["Y1_block"]
x2, y2 = r_last["X2_block"], r_last["Y2_block"]


# Новый конец отрезка с учетом изменения длины
x2_new = x1 
y2_new = y2 + delta_last
if delta_last > 0:
    # Удлинение — от старого конца к новому (зелёным)
    plt.plot([x2, x2_new], [y2, y2_new], color='red', linewidth=3)
    plt.text((x2 + x2_new)/2, (y2 + y2_new)/2, f'+{delta_last:.1f} мм', color='red', fontsize=9)

L_winch = rope_calc_params["L_winch"]   
L_winch_before = L_winch  

# Вытравливание каната у лебедки
try:
    if need_payout > 0:
        # Координаты лебёдки (первый участок каната начинается от неё)
        if rope_data:
            winch_x, winch_y = rope_data[0]["X1_block"], rope_data[0]["Y1_block"]
        else:
            # запасной вариант — центр 1-го блока
            winch_x, winch_y = blocks[0].coord.x, blocks[0].coord.y

        # Красная точка и подпись со старыми/новыми значениями L_winch
        plt.scatter([winch_x], [winch_y], color='red', s=40, zorder=6)
        txt = (
            f"L_winch: {L_winch_before:.1f} мм\n"
            f"L_winch_new: {new_L_winch:.1f} мм\n"
            f"−Δ = {need_payout:.1f} мм"
        )
        plt.text(
            winch_x-1000, winch_y + 1000,
            txt,
            color='red', fontsize=9, ha='left', va='bottom',
            bbox=dict(facecolor='white', edgecolor='red', alpha=0.5, boxstyle='round,pad=0.25')
        )
except Exception as e:
    print("[plot winch] Не удалось показать вытравливание у лебёдки:", e)
```
