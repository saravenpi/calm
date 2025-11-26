pub fn get_vimium_hints_script() -> &'static str {
    r#"
function installVimiumHints() {
    try {
    if (window.__CALM_VIMIUM_INSTALLED__) {
        return;
    }
    window.__CALM_VIMIUM_INSTALLED__ = true;

    const HINT_CHARS = 'asdfghjkl';
    let hintMode = null;
    let hintElements = [];
    let hintMarkers = [];
    let currentFilter = '';
    let searchMode = false;
    let searchBarVisible = false;
    let searchIndex = 0;
    let searchMatches = [];
    let currentSearchQuery = '';

    function isVisible(element) {
        const rect = element.getBoundingClientRect();
        if (rect.width === 0 || rect.height === 0) return false;
        if (rect.bottom < 0 || rect.top > window.innerHeight) return false;
        if (rect.right < 0 || rect.left > window.innerWidth) return false;

        const style = window.getComputedStyle(element);
        if (style.visibility === 'hidden' || style.display === 'none') return false;
        if (parseFloat(style.opacity) === 0) return false;

        return true;
    }

    function isClickable(element) {
        const tag = element.tagName.toLowerCase();
        if (['a', 'button', 'input', 'textarea', 'select', 'details', 'summary'].includes(tag)) {
            return true;
        }
        if (element.hasAttribute('onclick') || element.hasAttribute('ng-click')) {
            return true;
        }
        if (element.getAttribute('role') === 'button' || element.getAttribute('role') === 'link') {
            return true;
        }
        if (element.hasAttribute('contenteditable')) {
            return true;
        }
        if (element.hasAttribute('tabindex') && element.getAttribute('tabindex') !== '-1') {
            return true;
        }
        const cursor = window.getComputedStyle(element).cursor;
        if (cursor === 'pointer') {
            return true;
        }
        return false;
    }

    function getClickableElements() {
        const allElements = document.querySelectorAll('*');
        const clickable = [];

        for (let elem of allElements) {
            if (isClickable(elem) && isVisible(elem)) {
                clickable.push(elem);
            }
        }

        return clickable;
    }

    function generateHintStrings(count) {
        const hints = [];
        const chars = HINT_CHARS;

        if (count <= chars.length) {
            for (let i = 0; i < count; i++) {
                hints.push(chars[i]);
            }
            return hints;
        }

        let current = [''];
        while (current.length < count) {
            const newHints = [];
            for (let char of chars) {
                for (let hint of current) {
                    newHints.push(char + hint);
                    if (newHints.length >= count) break;
                }
                if (newHints.length >= count) break;
            }
            current = newHints;
        }

        return current.slice(0, count);
    }

    function createHintMarker(element, hintString) {
        const rect = element.getBoundingClientRect();
        const marker = document.createElement('div');

        marker.className = 'calm-hint-marker';
        marker.textContent = hintString;
        marker.style.cssText = `
            position: fixed;
            left: ${rect.left}px;
            top: ${rect.top}px;
            background: #000000;
            color: #ffffff;
            font-family: monospace;
            font-size: 12px;
            font-weight: bold;
            padding: 2px 4px;
            border: 1px solid #ffffff;
            border-radius: 2px;
            z-index: 2147483647;
            text-transform: uppercase;
            line-height: 1;
            pointer-events: auto;
            box-shadow: 0 2px 4px rgba(0,0,0,0.3);
        `;

        return marker;
    }

    function showHints(openInNewTab = false) {
        removeHints();

        hintMode = openInNewTab ? 'new-tab' : 'current-tab';
        currentFilter = '';

        const clickableElements = getClickableElements();

        if (clickableElements.length === 0) {
            return;
        }

        const hintStrings = generateHintStrings(clickableElements.length);

        const container = document.createElement('div');
        container.id = 'calm-hints-container';
        container.style.cssText = 'position: fixed; top: 0; left: 0; z-index: 2147483647; pointer-events: none;';
        document.body.appendChild(container);

        hintElements = [];
        hintMarkers = [];

        for (let i = 0; i < clickableElements.length; i++) {
            const element = clickableElements[i];
            const hintString = hintStrings[i];
            const marker = createHintMarker(element, hintString);

            container.appendChild(marker);
            hintElements.push({ element, hint: hintString, marker });
            hintMarkers.push(marker);
        }
    }

    function removeHints() {
        const container = document.getElementById('calm-hints-container');
        if (container) {
            container.remove();
        }
        hintMode = null;
        hintElements = [];
        hintMarkers = [];
        currentFilter = '';
    }

    function filterHints(input) {
        currentFilter += input.toLowerCase();
        let matchedElement = null;
        let visibleCount = 0;

        for (let item of hintElements) {
            const hintLower = item.hint.toLowerCase();

            if (hintLower.startsWith(currentFilter)) {
                item.marker.style.display = 'block';
                visibleCount++;

                if (hintLower === currentFilter) {
                    matchedElement = item.element;
                }

                const matchedPart = item.hint.substring(0, currentFilter.length);
                const remainingPart = item.hint.substring(currentFilter.length);
                item.marker.innerHTML = `<span style="opacity: 0.5">${matchedPart}</span>${remainingPart}`;
            } else {
                item.marker.style.display = 'none';
            }
        }

        if (matchedElement) {
            activateElement(matchedElement);
            removeHints();
        } else if (visibleCount === 0) {
            removeHints();
        }
    }

    function activateElement(element) {
        if (hintMode === 'new-tab') {
            if (element.tagName.toLowerCase() === 'a' && element.href) {
                window.ipc.postMessage(JSON.stringify({
                    action: 'open_url_new_tab',
                    url: element.href
                }));
            } else {
                element.click();
            }
        } else {
            element.click();
        }
    }

    function startSearch() {
        searchMode = true;
        searchBarVisible = true;
        searchIndex = 0;
        searchMatches = [];
        currentSearchQuery = '';

        const existingBar = document.getElementById('calm-search-bar');
        if (existingBar) existingBar.remove();

        const searchBar = document.createElement('div');
        searchBar.id = 'calm-search-bar';
        searchBar.style.cssText = `
            position: fixed;
            bottom: 20px;
            right: 20px;
            background: #000000;
            color: #ffffff;
            font-family: monospace;
            font-size: 14px;
            padding: 8px 12px;
            border: 2px solid #ffffff;
            z-index: 2147483647;
        `;

        const input = document.createElement('input');
        input.id = 'calm-search-input';
        input.type = 'text';
        input.placeholder = 'Search...';
        input.style.cssText = `
            background: #000000;
            color: #ffffff;
            border: 1px solid #ffffff;
            font-family: monospace;
            font-size: 14px;
            padding: 4px 8px;
            outline: none;
            width: 300px;
        `;

        const counter = document.createElement('span');
        counter.id = 'calm-search-counter';
        counter.style.cssText = `
            margin-left: 10px;
            color: #ffff00;
        `;

        searchBar.appendChild(input);
        searchBar.appendChild(counter);
        document.body.appendChild(searchBar);

        input.focus();

        input.addEventListener('input', (e) => {
            performSearch(e.target.value);
        });

        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                if (searchMatches.length > 0) {
                    searchBarVisible = false;
                    searchBar.remove();
                }
            } else if (e.key === 'Escape') {
                e.preventDefault();
                endSearch();
            }
        });
    }

    function performSearch(query) {
        clearHighlights();
        searchMatches = [];
        searchIndex = 0;
        currentSearchQuery = query;

        if (!query) {
            updateSearchCounter();
            return;
        }

        const walker = document.createTreeWalker(
            document.body,
            NodeFilter.SHOW_TEXT,
            {
                acceptNode: function(node) {
                    if (node.parentElement.id === 'calm-search-bar') {
                        return NodeFilter.FILTER_REJECT;
                    }
                    return NodeFilter.FILTER_ACCEPT;
                }
            }
        );

        const regex = new RegExp(query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi');
        let node;

        while (node = walker.nextNode()) {
            const text = node.textContent;
            let match;

            while ((match = regex.exec(text)) !== null) {
                searchMatches.push({
                    node: node,
                    index: match.index,
                    length: query.length
                });
            }
        }

        highlightMatches();
        if (searchMatches.length > 0) {
            scrollToMatch(0);
        }
        updateSearchCounter();
    }

    function highlightMatches() {
        for (let i = 0; i < searchMatches.length; i++) {
            const match = searchMatches[i];
            const node = match.node;
            const parent = node.parentElement;

            if (parent.classList.contains('calm-search-highlight')) continue;

            const before = node.textContent.substring(0, match.index);
            const matched = node.textContent.substring(match.index, match.index + match.length);
            const after = node.textContent.substring(match.index + match.length);

            const span = document.createElement('span');
            span.className = 'calm-search-highlight';
            span.style.cssText = 'background: #ffff00; color: #000000;';
            span.textContent = matched;

            const fragment = document.createDocumentFragment();
            if (before) fragment.appendChild(document.createTextNode(before));
            fragment.appendChild(span);
            if (after) fragment.appendChild(document.createTextNode(after));

            parent.replaceChild(fragment, node);
        }
    }

    function clearHighlights() {
        const highlights = document.querySelectorAll('.calm-search-highlight');
        for (let highlight of highlights) {
            const parent = highlight.parentElement;
            const text = highlight.textContent;
            parent.replaceChild(document.createTextNode(text), highlight);
            parent.normalize();
        }
    }

    function scrollToMatch(index) {
        if (searchMatches.length === 0) return;

        const allHighlights = document.querySelectorAll('.calm-search-highlight');
        allHighlights.forEach(h => h.style.background = '#ffff00');

        if (allHighlights[index]) {
            allHighlights[index].style.background = '#ff9900';
            allHighlights[index].scrollIntoView({ behavior: 'smooth', block: 'center' });
        }

        searchIndex = index;
        updateSearchCounter();
    }

    function nextSearchMatch() {
        if (searchMatches.length === 0) return;
        searchIndex = (searchIndex + 1) % searchMatches.length;
        scrollToMatch(searchIndex);
    }

    function prevSearchMatch() {
        if (searchMatches.length === 0) return;
        searchIndex = (searchIndex - 1 + searchMatches.length) % searchMatches.length;
        scrollToMatch(searchIndex);
    }

    function updateSearchCounter() {
        const counter = document.getElementById('calm-search-counter');
        if (counter) {
            if (searchMatches.length > 0) {
                counter.textContent = `${searchIndex + 1}/${searchMatches.length}`;
            } else {
                counter.textContent = '0/0';
            }
        }
    }

    function endSearch() {
        searchMode = false;
        searchBarVisible = false;
        clearHighlights();
        searchMatches = [];
        currentSearchQuery = '';
        const searchBar = document.getElementById('calm-search-bar');
        if (searchBar) searchBar.remove();
    }

    window.calmShowHints = showHints;
    window.calmRemoveHints = removeHints;
    window.calmFilterHints = filterHints;
    window.calmStartSearch = startSearch;
    window.calmEndSearch = endSearch;
    window.calmNextSearchMatch = nextSearchMatch;
    window.calmPrevSearchMatch = prevSearchMatch;
    window.calmGetHintMode = () => hintMode;
    window.calmIsSearchMode = () => searchMode;
    window.calmIsSearchBarVisible = () => searchBarVisible;

    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            if (hintMode) {
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation();
                removeHints();
                return false;
            } else if (searchMode) {
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation();
                endSearch();
                return false;
            } else if (document.activeElement &&
                      (document.activeElement.tagName === 'INPUT' ||
                       document.activeElement.tagName === 'TEXTAREA' ||
                       document.activeElement.tagName === 'SELECT' ||
                       document.activeElement.isContentEditable)) {
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation();
                document.activeElement.blur();
                return false;
            }
        }

        if (hintMode && e.key.length === 1 && e.key.match(/[a-z]/i)) {
            e.preventDefault();
            e.stopPropagation();
            e.stopImmediatePropagation();
            filterHints(e.key.toLowerCase());
            return false;
        }

        if (searchMode && !searchBarVisible) {
            if (e.key === 'n' && !e.shiftKey) {
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation();
                nextSearchMatch();
                return false;
            } else if (e.key === 'N' || (e.key === 'n' && e.shiftKey)) {
                e.preventDefault();
                e.stopPropagation();
                e.stopImmediatePropagation();
                prevSearchMatch();
                return false;
            }
        }
    }, true);

    } catch(e) {
        console.error('[CALM VIMIUM] Error in installVimiumHints:', e);
    }
}

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', installVimiumHints);
} else {
    installVimiumHints();
}
    "#
}
