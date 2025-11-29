Run sw-checklist on all components and analyze the results.

Execute `./scripts/check-all.sh` and then:

1. Report the total FAIL and WARN counts
2. List all FAIL items grouped by component
3. List all WARN items grouped by component
4. Identify which modules exceed function count limits (>7 functions)
5. Identify which functions exceed LOC limits (>50 lines)
6. Suggest the highest priority item to fix next based on:
   - FAILs before WARNs
   - Higher function counts first
   - Modules that can be fixed with the accessors/mutators/rendering pattern

After analysis, ask if I want to proceed with fixing the highest priority item.
