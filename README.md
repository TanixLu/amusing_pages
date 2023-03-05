# fun_page

## 开发笔记

### 更改public-url

projects的gh-pages不是在`/`下部署的，而是在`/project_name/`下部署的，所以在index.html的head中加入base标签：

```html
<base data-trunk-public-url />
```

build使加入--public-url参数：

```bash
trunk build --release --public-url="/fun_page/"
```
