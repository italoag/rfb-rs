# Demo Usage Examples

## Complete Workflow

```bash
# 1. Download data
rfb download --directory data --parallel 4 --skip-existing

# 2. Check integrity
rfb check --directory data

# 3. Transform data
rfb transform --directory data --output output --privacy

# 4. Create database
export DATABASE_URL="postgres://localhost/rfb"
rfb db create

# 5. Start API
rfb api --port 8080

# 6. Query data
curl http://localhost:8080/api/cnpj/00000000000191
```

See IMPLEMENTATION_COMPLETE.md for detailed examples.
