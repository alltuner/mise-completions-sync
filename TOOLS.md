# Tool Support Status

This document tracks shell completion support status for tools in mise's registry.
Use this to identify tools that could be added to mise-completions-sync.

## Supported Tools

Currently **77 tools** have completion support.

See [docs/tools.md](docs/tools.md) for the full list with shell support details.

## Pending Tools (Candidates)

These tools are in mise's registry but not yet supported. They may have completion support.

| Tool | Description | Status |
|------|-------------|--------|
| [1password](https://github.com/1password/cli) | Password manager developed by AgileBits Inc | Needs testing |
| aapt2 | Android Asset Packaging Tool (aapt) | Needs testing |
| [act](https://github.com/nektos/act) | Run your GitHub Actions locally | Needs testing |
| [action-validator](https://github.com/mpalmer/action-validator) | Tool to validate GitHub Action and Workflow YAM... | Needs testing |
| [actionlint](https://github.com/rhysd/actionlint) | :octocat: Static checker for GitHub Actions wor... | Needs testing |
| [adr-tools](https://github.com/npryce/adr-tools) | Command-line tools for working with Architectur... | Needs testing |
| ag | The Silver Searcher: A code searching tool simi... | Needs testing |
| [age](https://github.com/FiloSottile/age) | A simple, modern and secure encryption tool (an... | Needs testing |
| age-plugin-yubikey | age-plugin-yubikey is a plugin for age clients ... | Needs testing |
| [agebox](https://github.com/slok/agebox) | Age based repository file encryption gitops tool | Needs testing |
| [aichat](https://github.com/sigoden/aichat) | Use GPT-4(V), Gemini, LocalAI, Ollama and other... | Needs testing |
| [aks-engine](https://github.com/Azure/aks-engine) | AKS Engine deploys and manages Kubernetes clust... | Needs testing |
| allure | Allure Report is a popular open source tool for... | Needs testing |
| allurectl | allurectl is the command line wrapper of Allure... | Needs testing |
| [alp](https://github.com/tkuchiki/alp) | Access Log Profiler | Needs testing |
| amass | In-depth attack surface mapping and asset disco... | Needs testing |
| [amazon-ecr-credential-helper](https://github.com/awslabs/amazon-ecr-credential-helper) | Automatically gets credentials for Amazon ECR o... | Needs testing |
| [amazon-ecs-cli](https://github.com/aws/amazon-ecs-cli) | The Amazon ECS CLI enables users to run their a... | Needs testing |
| amp | An agentic coding tool built by Sourcegraph | Needs testing |
| android-sdk | Android Command-line tools | Needs testing |
| ansible | ansible python package contains the core runtim... | Needs testing |
| ansible-core | ansible-core python package contains the core r... | Needs testing |
| ant | Apache Ant is a Java library and command-line t... | Needs testing |
| [apko](https://github.com/chainguard-dev/apko) | Build OCI images from APK packages directly wit... | Needs testing |
| apollo-ios | Apollo iOS Code Generation | Needs testing |
| apollo-router | A configurable, high-performance routing runtim... | Needs testing |
| apollo-rover | The CLI for Apollo GraphOS | Needs testing |
| aqua | Declarative CLI Version manager written in Go. ... | Needs testing |
| [arduino](https://github.com/arduino/arduino-cli) | Arduino command line tool | Needs testing |
| argc | A Bash CLI framework, also a Bash command runner | Needs testing |
| [argo](https://github.com/argoproj/argo-workflows) | Argo Workflows CLI. Workflow engine for Kubernetes | Needs testing |
| [argo-rollouts](https://github.com/argoproj/argo-rollouts) | Progressive Delivery for Kubernetes | Needs testing |
| asciidoctorj | AsciidoctorJ is the official library for runnin... | Needs testing |
| assh | make your ssh client smarter | Needs testing |
| [ast-grep](https://github.com/ast-grep/ast-grep) | A CLI tool for code structural search, lint and... | Needs testing |
| astro | CLI that makes it easy to create, test and depl... | Needs testing |
| [atlas](https://github.com/ariga/atlas) | A modern tool for managing database schemas | Needs testing |
| [atmos](https://github.com/cloudposse/atmos) | Workflow automation tool for DevOps. Keep confi... | Needs testing |
| auto-doc | Github action that turns your reusable workflow... | Needs testing |
| aws-amplify | The AWS Amplify CLI is a toolchain for simplify... | Needs testing |
| [aws-cli](https://github.com/aws/aws-cli) | The AWS Command Line Interface (AWS CLI v2) is ... | Needs testing |
| [aws-copilot](https://github.com/aws/copilot-cli) | The AWS Copilot CLI is a tool for developers to... | Needs testing |
| [aws-iam-authenticator](https://github.com/kubernetes-sigs/aws-iam-authenticator) | A tool to use AWS IAM credentials to authentica... | Needs testing |
| [aws-nuke](https://github.com/ekristen/aws-nuke) | Remove all the resources from an AWS account | Needs testing |
| [aws-sam](https://github.com/aws/aws-sam-cli) | CLI tool to build, test, debug, and deploy Serv... | Needs testing |
| [aws-sso](https://github.com/synfinatic/aws-sso-cli) | A powerful tool for using AWS Identity Center f... | Needs testing |
| [aws-vault](https://github.com/ByteNess/aws-vault) | A vault for securely storing and accessing AWS ... | Needs testing |
| awscli-local | This package provides the awslocal command, whi... | Needs testing |
| awsebcli | The AWS Elastic Beanstalk Command Line Interfac... | Needs testing |
| awsls | A list command for AWS resources | Needs testing |

*...and 832 more tools in mise registry*

## Tools Without Completion Support

These tools have been tested and confirmed to NOT output shell completion scripts:

- **air**: Live reload for Go apps
- **aws**: The AWS Command Line Interface (AWS CLI v2) is ...
- **biome**: A toolchain for web projects, aimed to provide ...
- **consul**: Consul is a distributed, highly available, and ...
- **croc**: Easily and securely send things from one comput...
- **dust**: A more intuitive version of du in rust
- **evans**: Evans: more expressive universal gRPC client
- **eza**: A modern, maintained replacement for ls
- **nomad**: Nomad is an easy-to-use, flexible, and performa...
- **terraform**: Terraform enables you to safely and predictably...
- **tokei**: Count your code, quickly
- **vault**: A tool for secrets management, encryption as a ...
- **wrangler**: A command line tool for building Cloudflare Wor...
- **yarn**: Yarn is a package manager that doubles down as ...

## How to Add a Tool

1. Check if the tool supports shell completions:
   ```bash
   mise x <tool> -- <tool> completion --help
   mise x <tool> -- <tool> --help | grep -i complet
   ```

2. Find the correct completion command pattern

3. Add entry to `registry.toml`:
   ```toml
   # If it matches an existing pattern:
   newtool = "standard"  # for: newtool completion <shell>

   # Or with explicit commands:
   newtool = { zsh = "newtool completions zsh", bash = "newtool completions bash" }
   ```

4. Test the entry:
   ```bash
   uv run scripts/validate-registry.py --installed-only
   ```

5. Submit a PR
