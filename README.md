# fun_page

## 开发笔记

### 更改public-url，以及gh-pages SPA

projects的gh-pages不是在`/`下部署的，而是在`/project_name/`下部署的，所以在index.html的head中加入base标签：

```html
<base data-trunk-public-url />
```

build时加入--public-url参数，并且加入404.html、.nojekyll，这是为了在gh-pages创建SPA应用：

```bash
trunk build --release --public-url="/fun_page/" ; cp 404.html dist/404.html ; echo $null >> dist/.nojekyll
```

### 将dist提交到gh-pages分支

首先需要建立孤立的gh-pages分支：

```bash
git checkout --orphan gh-pages
git reset --hard
git commit --allow-empty -m "init"
git push origin gh-pages
git checkout main
```

然后执行：

```bash
git add -f dist
git commit -m "dist"
git subtree push --prefix dist origin gh-pages
git reset HEAD~1
git rm -r --cached dist
```

如果报错，可尝试用一个temp分支解决：

```bash
git add -f dist
git commit -m "dist"
git subtree split --prefix dist -b temp
git push -f origin temp:gh-pages
git branch -D temp
git reset HEAD~1
git rm -r --cached dist
```

### 将dist提交到gh-pages分支

首先需要建立孤立的gh-pages分支：

```bash
git checkout --orphan gh-pages
git reset --hard
git commit --allow-empty -m "init"
git push origin gh-pages
git checkout main
```

在main分支内需要将dist commit上，然后执行：

```bash
git subtree push --prefix dist origin gh-pages
```

如果报错，可尝试用一个temp分支解决：

```bash
git subtree split --prefix dist -b temp
git push -f origin temp:gh-pages
git brandh -D temp
```

最后回退main分支到未提交dist状态：

```bash
git reset HEAD~1
git rm -r --cached dist
```
