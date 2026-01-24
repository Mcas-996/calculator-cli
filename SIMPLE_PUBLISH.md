# 发布 calculator-cli 到 npm

## 名称更改

由于 `calculator-cli` 包名已有冲突，我们已将包名更改为 `mathcalc-cli`。

## 发布脚本

```json
{
  "name": "mathcalc-cli",
  "version": "0.2.0",
  ...
}
```

## 发布步骤

1. 登录 npm（如果尚未登录）
   ```bash
   npm login
   ```

2. 创建包
   ```bash
   npm pack
   ```

3. 发布
   ```bash
   npm publish
   ```

## 用户安装方式

安装后即可使用：
```bash
npm install -g mathcalc-cli
mathcalc "2 + 2"
```

## 备选方案

如果名称仍不可用，可尝试：
```json
{
  "name": "@mcas-996/calculator",
  "publishConfig": { "registry": "https://registry.npmjs.org/" }
}
```
