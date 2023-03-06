# amusing_pages

## 开发笔记

### 更改public-url，以及gh-pages SPA

projects的gh-pages不是在`/`下部署的，而是在`/project_name/`下部署的，所以在index.html的head中加入base标签：

```html
<base data-trunk-public-url />
```

build时加入--public-url参数，并且将spa_dist文件夹中的文件复制进去（因为build会请问dist文件夹），这是为了在gh-pages创建SPA应用：

```bash
trunk build --release --public-url="/amusing_pages/" ; cp -r spa_dist/* dist
```

感谢[spa-github-pages](https://github.com/rafgraph/spa-github-pages)提出的gh-pages SPA解决方法❤

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
