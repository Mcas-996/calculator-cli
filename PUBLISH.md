# 发布 calculator-cli 到 npm

## 当前问题

npm 显示 `calculator-cli` 包名与 `calculator_cli` 包名太相似，需要更改包名。

## 解决方案

当前的 package.json 已经更改为：
- **包名**: `mathcalc-cli`
- **版本**: `0.2.0`
- **仓库地址**: https://github.com/Mcas-996/calculator-cli.git

## 发布步骤

1. 检查当前 package.json
   ```bash
   cat package.json
   ```

2. 创建测试包
   ```bash
   npm pack
   ```

3. 验证包内容
   ```bash
   tar -tf mathcalc-cli-0.1.0.tgz
   ```

4. 登录 npm（如果尚未登录）
   ```bash
   npm login
   ```

5. 发布包
   ```bash
   npm publish
   ```

## 安装测试

安装包：
```bash
npm install -g mathcalc-cli
```

测试命令：
```bash
mathcalc "2 + 2"
mathcalc "x^2-4=0"
```

## 包命名解释

选择 `mathcalc-cli` 的原因：
- 名称独特，不会与现有包冲突
- 清晰表示功能：Math Calculator CLI
- 简短易记

## 用户安装后使用

安装后，用户可以在任何位置运行：
```bash
mathcalc "expression"
```

## 如果仍有问题

1. 检查 npm 账户权限
2. 尝试使用带作用域的名称
3. 使用 @username/package-name 格式
   ```bash
   npm publish --access=public
   ```

## 备选包名

如果 `mathcalc-cli` 仍被占用，可以使用：

- `mathcalc-toolkit`
- `rustcalculator`
- `symbolic-calc`
- `calc-cli`

要使用这些名称，只需更改 package.json 中的 "name" 字段。
```

现在让我删除之前创建的复杂脚本文件，简化为最基本的构建和发布流程。我将使用 `overwrite` 模式重写整个文件内容。</think>
```md
# 发布 calculator-cli v0.1.0 到 npm

## 当前状态

包名已经从 `calculator-cli` 更改为 `mathcalc-cli` 避免与 `calculator_cli` 名称冲突。

## 发布步骤

1. **检查当前配置**
   ```bash
   cat package.json
   ```

2. **创建压缩包**
   ```bash
   npm pack
   ```

3. **验证包内容**
   ```bash
   tar -tf mathcalc-cli-0.1.0.tgz
   ```

4. **登录到npm**（如需要）
   ```bash
   npm login
   ```

5. **发布包**
   ```bash
   npm publish
   ```

## 用户安装方式

安装后的版本使用命令：
```bash
# 全局安装
npm install -g mathcalc-cli

# 运行计算
mathcalc "2 + 2"  # 4
mathcalc "x^2-4=0"  # x = 2, -2
```

## 替代安装方法

如果npm注册的名称不可用，用户也可以：

1. 直接从GitHub Release下载
2. 克隆仓库并使用二进制文件：
   ```bash
   https://github.com/Mcas-996/calculator-cli/releases
   git clone https://github.com/Mcas-996/calculator-cli.git
   ```

## 包名称更改说明

从 `calculator-cli` 更改为 `mathcalc-cli` 后：

- npm 包名已更新
- 二进制文件名保持不变
- 功能完全相同
- 仅安装方式略有不同

## 用户使用体验

无论使用哪种安装方式，用户都可以：

- 在命令行任何位置运行 `mathcalc`
- 享受相同的数学计算功能
- 获得精确分数和符号结果

## 更新说明

如果您需要回到 `calculator-cli` 名称：

```bash
# 修改 package.json 中的 name 字段
# 运行 npm unpublish（如果已发布）
# 重新发布
npm unpublish mathcalc-cli
# 修改 package.json 为 "calculator-cli"
npm publish --access public
```

现在让我删除之前创建的复杂脚本文件，简化为最基本的构建和发布流程。我将使用 `overwrite` 模式重写整个文件内容。
