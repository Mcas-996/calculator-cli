# GitLab CI/CD Setup Guide for calculator-cli

This guide explains how to set up GitLab CI/CD to build and publish the calculator-cli package to npm.

## Prerequisites

1. A GitLab repository with the calculator-cli project
2. An npm account with permission to publish packages
3. GitLab Runner access (shared runners work for Linux, self-hosted for macOS/Windows if needed)

## Setting up npm Authentication

1. Create an npm access token:
   - Log in to [npmjs.com](https://www.npmjs.com)
   - Go to Account > Access Tokens
   - Click "Generate New Token"
   - Select "Automation" level
   - Copy the generated token

2. Add the token to GitLab CI/CD variables:
   - In your GitLab project, go to Settings > CI/CD
   - Expand the Variables section
   - Add a new variable:
     - Key: `NPM_TOKEN`
     - Value: [paste your npm token]
     - Protect variable: Check if needed
     - Mask variable: Check for security

## GitLab Runner Requirements

### Linux (shared runners)
- No special requirements, uses GitLab's shared runners

### macOS (self-hosted runner)
- Install GitLab Runner on a macOS machine
- Register the runner with tags: `macos`
- Install Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install Xcode Command Line Tools: `xcode-select --install`

### Windows (self-hosted runner)
- Install GitLab Runner on a Windows machine
- Register the runner with tags: `windows`
- Install Rust toolchain: https://rustup.rs/
- Install Microsoft Visual Studio with C++ build tools

## Release Process

1. Tag a release with semantic versioning:
   ```bash
   git tag -a v2.0.1 -m "Release version 2.0.1"
   git push origin v2.0.1
   ```

2. GitLab CI/CD will automatically:
   - Build binaries for all platforms
   - Publish the package to npm
   - Create a GitLab release

## Troubleshooting

### Build failures on macOS/Windows
- Ensure self-hosted runners are properly registered
- Verify that target platforms are installed: `rustup target add x86_64-apple-darwin` or `rustup target add x86_64-pc-windows-msvc`

### npm publish failures
- Verify NPM_TOKEN is correctly set in GitLab CI/CD variables
- Ensure you have publish permissions for the npm package
- Check if the package name is available on npm

### Build artifact issues
- Verify that bin directory is properly created
- Check that all artifact paths are correct
- Ensure artifacts expire at appropriate times

## Advanced Configuration

### Automatic release notes
Add to `.gitlab-ci.yml`:
```yaml
create-release:
  stage: publish
  image: registry.gitlab.com/gitlab-org/release-cli:latest
  needs:
    - publish
  script:
    - echo "Creating release..."
  release:
    tag_name: $CI_COMMIT_TAG
    description: "Release $CI_COMMIT_TAG"
  rules:
    - if: $CI_COMMIT_TAG
```

### Multiple environments
Add environment-specific variables:
- `NPM_TOKEN_STAGING` for testing
- `NPM_TOKEN_PRODUCTION` for production
- Modify publish job to use appropriate token based on branch or tag

## Migration from GitHub Actions

If migrating from GitHub Actions:
1. Replace `.github/workflows/` with `.gitlab-ci.yml`
2. Update secret references from `${{ secrets.NPM_TOKEN }}` to `${NPM_TOKEN}`
3. Adjust matrix builds to separate jobs with GitLab's needs: keyword
4. Update artifact handling to use GitLab's artifacts system

## Monitoring and Maintenance

- Monitor CI/CD pipeline success rates
- Check npm package for successful updates
- Review build logs periodically for issues
- Update runner configurations as needed
- Keep Rust toolchain versions up to date