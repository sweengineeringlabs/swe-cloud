#!/usr/bin/env bash
# bin/validate-routes.sh — validate page references in route YAML files
source "$(cd "$(dirname "$0")/.." && pwd)/lib/common.sh"

cd "$UI_DIR"

echo "${BOLD}Route Mapping Validator${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# ── Route files to validate ──────────────────────────────────────────
ROUTE_FILES=(
    "routes.yaml"
    "src/features/cloudemu/cloudemu_routes.yaml"
    "src/features/cloudkit/cloudkit_routes.yaml"
    "src/features/iac/iac_routes.yaml"
)

ERRORS=0
WARNINGS=0
PAGES_CHECKED=0

# ── Helpers ──────────────────────────────────────────────────────────
extract_pages() {
    local file=$1
    if [ -f "$file" ]; then
        grep -E "^\s*page:" "$file" 2>/dev/null | sed 's/.*page:\s*//' | tr -d '"' | tr -d "'" | xargs
    fi
}

check_page_file() {
    local page=$1
    [ -f "src/pages/${page}.page.rsx" ]
}

# ── Validate each route file ────────────────────────────────────────
for route_file in "${ROUTE_FILES[@]}"; do
    if [ ! -f "$route_file" ]; then
        echo -e "${YELLOW}⚠ Route file not found: $route_file${NC}"
        ((WARNINGS++))
        continue
    fi

    echo "→ Validating $route_file"

    pages=$(extract_pages "$route_file")

    for page in $pages; do
        ((PAGES_CHECKED++))

        # Check for kebab-case (invalid)
        if [[ "$page" == *"-"* ]]; then
            echo -e "  ${RED}✗ Invalid naming (kebab-case): $page${NC}"
            echo -e "    Expected snake_case: ${page//-/_}"
            ((ERRORS++))
            continue
        fi

        # Check file exists
        if check_page_file "$page"; then
            echo -e "  ${GREEN}✓${NC} $page"
        else
            echo -e "  ${RED}✗ Missing page file: src/pages/${page}.page.rsx${NC}"
            ((ERRORS++))
        fi
    done

    echo ""
done

# ── Check for orphan page files ─────────────────────────────────────
echo "→ Checking for orphan page files"
ORPHANS=0

for file in src/pages/*.page.rsx; do
    if [ -f "$file" ]; then
        basename=$(basename "$file" .page.rsx)

        # Skip known standalone pages
        if [[ "$basename" == "dashboard" || "$basename" == "not_found" ]]; then
            continue
        fi

        # Check if page is referenced in any route file
        found=false
        for route_file in "${ROUTE_FILES[@]}"; do
            if [ -f "$route_file" ] && grep -q "page:\s*[\"']*${basename}[\"']*" "$route_file"; then
                found=true
                break
            fi
        done

        if [ "$found" = false ]; then
            echo -e "  ${YELLOW}⚠ Orphan page (no route): $basename${NC}"
            ((ORPHANS++))
        fi
    fi
done

if [ $ORPHANS -eq 0 ]; then
    echo -e "  ${GREEN}✓ No orphan pages${NC}"
fi

# ── Summary ──────────────────────────────────────────────────────────
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Summary:"
echo "  Pages checked: $PAGES_CHECKED"
echo "  Errors: $ERRORS"
echo "  Warnings: $WARNINGS"
echo "  Orphans: $ORPHANS"
echo ""

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}✗ Validation failed with $ERRORS error(s)${NC}"
    exit 1
else
    echo -e "${GREEN}✓ All route mappings valid${NC}"
    exit 0
fi
