# Supported Tools

The following tools have completion support in mise-completions-sync.

| Tool | ZSH | Bash | Fish |
|------|-----|------|------|
| age | ✓ | ✓ | ✓ |
| air | ✓ | ✓ | ✓ |
| argocd | ✓ | ✓ | ✓ |
| atuin | ✓ | ✓ | ✓ |
| aws | ✓ | ✓ |  |
| bat | ✓ | ✓ | ✓ |
| biome | ✓ | ✓ | ✓ |
| bun | ✓ |  |  |
| cargo | ✓ | ✓ | ✓ |
| chezmoi | ✓ | ✓ | ✓ |
| consul | ✓ | ✓ |  |
| cosign | ✓ | ✓ | ✓ |
| croc | ✓ | ✓ | ✓ |
| cue | ✓ | ✓ | ✓ |
| dagger | ✓ | ✓ | ✓ |
| delta | ✓ | ✓ | ✓ |
| deno | ✓ | ✓ | ✓ |
| docker | ✓ | ✓ | ✓ |
| doctl | ✓ | ✓ | ✓ |
| dust | ✓ | ✓ | ✓ |
| evans | ✓ | ✓ | ✓ |
| eza | ✓ | ✓ | ✓ |
| fd | ✓ | ✓ | ✓ |
| flux | ✓ | ✓ | ✓ |
| flyctl | ✓ | ✓ | ✓ |
| fzf | ✓ | ✓ | ✓ |
| gcloud | ✓ | ✓ |  |
| gh | ✓ | ✓ | ✓ |
| gitu | ✓ | ✓ | ✓ |
| gitui | ✓ | ✓ | ✓ |
| glab | ✓ | ✓ | ✓ |
| go | ✓ | ✓ |  |
| golangci-lint | ✓ | ✓ | ✓ |
| goreleaser | ✓ | ✓ | ✓ |
| grpcurl | ✓ | ✓ | ✓ |
| helm | ✓ | ✓ | ✓ |
| hugo | ✓ | ✓ | ✓ |
| hyperfine | ✓ | ✓ | ✓ |
| istioctl | ✓ | ✓ | ✓ |
| just | ✓ | ✓ | ✓ |
| k3d | ✓ | ✓ | ✓ |
| k9s | ✓ | ✓ | ✓ |
| kind | ✓ | ✓ | ✓ |
| ko | ✓ | ✓ | ✓ |
| krew | ✓ | ✓ | ✓ |
| kubectl | ✓ | ✓ | ✓ |
| kubectx | ✓ | ✓ |  |
| kubeseal | ✓ | ✓ | ✓ |
| kustomize | ✓ | ✓ | ✓ |
| lazygit | ✓ | ✓ | ✓ |
| lefthook | ✓ | ✓ | ✓ |
| linkerd | ✓ | ✓ | ✓ |
| mdbook | ✓ | ✓ | ✓ |
| minikube | ✓ | ✓ | ✓ |
| mise | ✓ | ✓ | ✓ |
| mkcert | ✓ | ✓ | ✓ |
| nerdctl | ✓ | ✓ | ✓ |
| nix | ✓ | ✓ | ✓ |
| nomad | ✓ | ✓ |  |
| npm | ✓ | ✓ |  |
| oc | ✓ | ✓ | ✓ |
| oci | ✓ | ✓ | ✓ |
| pdm | ✓ | ✓ | ✓ |
| pipx | ✓ | ✓ |  |
| pnpm | ✓ | ✓ |  |
| podman | ✓ | ✓ | ✓ |
| poetry | ✓ | ✓ | ✓ |
| pulumi | ✓ | ✓ | ✓ |
| restic | ✓ | ✓ | ✓ |
| rg | ✓ | ✓ | ✓ |
| ripgrep | ✓ | ✓ | ✓ |
| ruff | ✓ | ✓ | ✓ |
| rustup | ✓ | ✓ | ✓ |
| saml2aws | ✓ | ✓ | ✓ |
| skaffold | ✓ | ✓ | ✓ |
| sops | ✓ | ✓ | ✓ |
| starship | ✓ | ✓ | ✓ |
| step | ✓ | ✓ | ✓ |
| stern | ✓ | ✓ | ✓ |
| task | ✓ | ✓ | ✓ |
| terraform | ✓ | ✓ |  |
| tilt | ✓ | ✓ | ✓ |
| tokei | ✓ | ✓ | ✓ |
| trivy | ✓ | ✓ | ✓ |
| ty | ✓ | ✓ | ✓ |
| uv | ✓ | ✓ | ✓ |
| vault | ✓ | ✓ |  |
| velero | ✓ | ✓ | ✓ |
| vercel | ✓ | ✓ |  |
| wrangler | ✓ | ✓ |  |
| xh | ✓ | ✓ | ✓ |
| yarn | ✓ |  |  |
| zellij | ✓ | ✓ | ✓ |
| zoxide | ✓ | ✓ | ✓ |

**Total: 94 tools**

## Adding Tools

If a tool you use isn't in the registry:

1. Check if the tool supports completions (usually `tool completion --help`)
2. Add an entry to `registry.toml`
3. Submit a PR
