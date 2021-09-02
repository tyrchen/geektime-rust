# 构建 Python 编译环境

创建 virtual env，然后用 `maturin develop` 构建：
```bash
cd queryer-py
python -m venv .env
source .env/bin/activate
pip install maturin
maturin develop
```

之后可以使用：

```ipython
In [1]: import queryer_py

In [2]: sql = queryer_py.example_sql()

In [3]: print(queryer_py.query(sql))
name,total_cases,new_cases,total_deaths,new_deaths
India,32649947.0,46759.0,437370.0,509.0
Iran,4869414.0,36279.0,105287.0,571.0
Africa,7695475.0,33957.0,193394.0,764.0
South America,36768062.0,33853.0,1126593.0,1019.0
Brazil,20703906.0,27345.0,578326.0,761.0
Mexico,3311317.0,19556.0,257150.0,863.0
```
