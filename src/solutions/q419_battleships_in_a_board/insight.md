# 战舰数量

## 思路

非常直白的相连元素问题。考虑到矩阵网格之间连通性很强，会有很深的递归深度，因此应当使用广度优先搜索划分相连区域。

## 踩坑指南

- 最重要的一点，对图来说，表示顶点的迭代器有两种可能的形式: 一种是 `Copy` 的，即题中这个情形，此时两个矩阵之间的连通可以通过其 `Copy` 的索引计算得到；另一种是引用的，需要其引用来得到连通关系（也是引用的形式）。如果使用传入闭包方法来提供连通关系的情况下，这两种情形下的函数签名不同，因此需要分别实现。
- 闭包函数在迭代器中使用时，迭代器的 FnMut 会取得闭包函数的所有权（即使迭代过程中实际上是只读的），因此应当最初就应当使用闭包函数的引用 `&P where P: Fn()->()`。
- 最后将整个图分割成若干个区域后，需要减去水域的数量。水域数量有可能是 0，有可能是 1，也有可能更多（战舰可以对角排列将水分割成两个及以上的部分），可以在分割之后判断其代表的类型，迭代排除所有水域，这里暂时没有想到偷懒/优化的方法。
