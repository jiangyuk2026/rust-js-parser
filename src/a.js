const fs = require('fs')
const path = require('path')
const parser = require('@babel/parser')
const t = require('@babel/types')
const recast = require('recast')
const Audio = require('./other/Audio')
const MyStorage = require('./other/MyStorage')

function getAst(code) {
    const ast = parser.parse(code, {
        sourceType: 'script',
        plugins: [],
        tokens: true,
        allowReturnOutsideFunction: true
    })
    return ast
}


a = (a = b + 3) => {
}

function b(c, {d: {e: [f, g, {h = 3}]}}) {}

let ast = getAst("")
let body = ast.program.body
console.log(ast)

function getNodeCode(node, limit = 100) {
    if (!node) {
        return
    }
    let before = node.leadingComments
    try {
        node.leadingComments = []
        return recast.print(node).code.substring(0, limit).replace('', '')
    } finally {
        node.leadingComments = before
    }
}

const vConsole = console

function getGlobalObj(window) {
    const globalObj = {
        NaN: window.NaN,
        Infinity: Infinity,
        isNaN: window.isNaN,
        isFinite: isFinite,
        setTimeout: window.setTimeout,
        setInterval: window.setInterval,
        clearTimeout: window.clearTimeout,
        clearInterval: window.clearInterval,
        requestAnimationFrame: window.setTimeout,
        cancelAnimationFrame: window.clearTimeout,
        dispatchEvent: window.dispatchEvent.bind(window),
        postMessage: window.postMessage.bind(window),
        Dispatch: window.Dispatch,
        Event: Event,
        localStorage: new MyStorage(),
        sessionStorage: new MyStorage(),
        setImmediate: setImmediate,
        clearImmediate: clearImmediate,
        Proxy: Proxy,
        Reflect: Reflect,
        MessageChannel: window.MessageChannel,
        document: window.document,
        URL: URL,
        URLSearchParams: URLSearchParams,
        Promise: Promise,
        navigator: {
            userAgent: 'Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Mobile Safari/537.36',
            platform: 'MacIntel',
            language: 'en-US',
            languages: ['en-US', 'en', 'zh-CN'],
            javaEnabled: () => false,
            plugins: []
        },
        screen: {
            width: 1000,
            height: 1000
        },
        location: window.location,
        parseFloat: window.parseFloat,
        parseInt: window.parseInt,
        encodeURI: encodeURI,
        decodeURI: decodeURI,
        escape: escape,
        unescape: unescape,
        crypto: window.crypto,
        encodeURIComponent: window.encodeURIComponent,
        decodeURIComponent: window.decodeURIComponent,
        addEventListener: window.addEventListener.bind(window),
        removeEventListener: window.removeEventListener.bind(window),
        getComputedStyle: window.getComputedStyle.bind(window),
        DOMParser: window.DOMParser,
        XMLHttpRequest: window.XMLHttpRequest,
        DOMTokenList: window.DOMTokenList,
        NodeList: window.NodeList,
        // CSSRuleList: window.CSSRuleList,
        CSSStyleDeclaration: window.CSSStyleDeclaration,
        // CSSValueList: window.CSSValueList,
        // ClientRectList: window.ClientRectList,
        // DOMRectList: window.DOMRectList,
        // DOMStringList: window.DOMStringList,
        // DataTransferItemList: window.DataTransferItemList,
        FileList: window.FileList,
        // HTMLAllCollection: window.HTMLAllCollection,
        HTMLCollection: window.HTMLCollection,
        HTMLFormElement: window.HTMLFormElement,
        HTMLSelectElement: window.HTMLSelectElement,
        MediaList: window.MediaList,
        MimeTypeArray: window.MimeTypeArray,
        NamedNodeMap: window.NamedNodeMap,
        // SVGLengthList: window.SVGLengthList,
        // SVGNumberList: window.SVGNumberList,
        // SVGPathSegList: window.SVGPathSegList,
        // SVGPointList: window.SVGPointList,
        SVGStringList: window.SVGStringList,
        // SVGTransformList: window.SVGTransformList,
        // SourceBufferList: window.SourceBufferList,
        HTMLIFrameElement: window.HTMLIFrameElement,
        Image: window.Image,
        Set: Set,
        WeakSet: WeakSet,
        Map: Map,
        WeakMap: WeakMap,
        Object: Object,
        Array: Array,
        Function: Function,
        Boolean: Boolean,
        Number: Number,
        String: String,
        Symbol: Symbol,
        Date: Date,
        ArrayBuffer: ArrayBuffer,
        DataView: DataView,
        Error: Error,
        TypeError: TypeError,
        RangeError: RangeError,
        SyntaxError: SyntaxError,
        ReferenceError: ReferenceError,
        AggregateError: AggregateError,
        Math: Math,
        RegExp: RegExp,
        console: vConsole,
        JSON: JSON,
        Int8Array: Int8Array,
        Int16Array: Int16Array,
        Int32Array: Int32Array,
        Uint8Array: Uint8Array,
        Uint16Array: Uint16Array,
        Uint32Array: Uint32Array,
        Float32Array: Float32Array,
        Float64Array: Float64Array,
        BigInt64Array: BigInt64Array,
        BigUint64Array: BigUint64Array,
        Uint8ClampedArray: Uint8ClampedArray,
        Audio: Audio,
        Blob: window.Blob,
        FileReader: window.FileReader,
        DOMException: window.DOMException,
        matchMedia: () => ({}),
        performance: window.performance,
        eval: () => {
            console.log('eval')
        },
    }
    return globalObj
}

const ignoreObjList = [
    RegExp.prototype, Object.prototype, Function.prototype, Date.prototype, Array.prototype, String.prototype, Number.prototype, Boolean.prototype, Symbol.prototype
]

function isIgnoreObjKey(obj, key) {
    let result = ignoreObjList.find(item => item === obj) !== undefined
    if (result) {
        // console.log('123123123', key)
    }
    return result
}

const functionTypeList = ['FunctionDeclaration', 'FunctionExpression', 'ArrowFunctionExpression']

function getElevationVariableList(node) {
    if (functionTypeList.find(item => item === node.type) !== undefined) {
        return []
    }
    const list = []
    let functionCount = 0
    t.traverse(node, {
        enter: (enterNode) => {
            if (functionTypeList.find(item => item === enterNode.type)) {
                functionCount++
                return
            }
            if (functionCount != 0) {
                return
            }
            if (enterNode.type === 'VariableDeclaration' && enterNode.kind === 'var') {
                list.push(enterNode)
            }
            if (enterNode.type === 'FunctionDeclaration') {
                list.push(enterNode)
            }
        },
        exit(exitNode) {
            if (functionTypeList.find(item => item === exitNode.type)) {
                functionCount--
            }
        }
    })
    return list
}

function getRunItem(filePath) {
    let parts = filePath.split('/')
    let name = parts[parts.length - 1]
    let nameParts = name.split(/[-.]/).slice(0, 2)
    return {
        fileName: nameParts.join('-'),
        content: fs.readFileSync(filePath).toString()
    }
}

function getNodeStr(node) {
    if (!node) {
        return ''
    }
    switch (node.type) {
        case 'ExpressionStatement':
            return getNodeStr(node.expression)
        case 'FunctionDeclaration':
            return `function ${node.id.name}(${node.params.map(getNodeStr).join(',')})` + getNodeStr(node.body)
        case 'FunctionExpression':
            return `function ${node.id ? node.id.name : ''}(${node.params.map(getNodeStr).join(',')})` + getNodeStr(node.body)
        case 'ArrowFunctionExpression':
            return `(${node.params.map(getNodeStr).join(',')})=>` + getNodeStr(node.body)
        case 'CallExpression':
            return getNodeStr(node.callee) + '(' + node.arguments.map(getNodeStr).join(',') + ')'
        case 'Identifier':
            return node.name
        case 'StringLiteral':
        case 'BooleanLiteral':
            return `"${node.value}"`
        case 'NumericLiteral':
            return node.value
        case 'ThisExpression':
            return 'this'
        case 'NullLiteral':
            return 'null'
        case 'BlockStatement':
            return ' { ' + node.body.slice(0, 2).map(getNodeStr) + `${node.body.length > 2 ? ' ,... } ' : ' } '}`
        case 'RegExpLiteral':
            return '/' + node.pattern + '/' + node.flags
        case 'VariableDeclaration':
            return node.kind + ' ' + node.declarations.map(getNodeStr).join(', ')
        case 'VariableDeclarator':
            return getNodeStr(node.id) + (node.init ? (' = ' + getNodeStr(node.init)) : '')
        case 'LogicalExpression':
        case 'AssignmentExpression':
        case 'BinaryExpression':
            return getNodeStr(node.left) + ' ' + node.operator + ' ' + getNodeStr(node.right)
        case 'MemberExpression': {
            if (node.computed) {
                return getNodeStr(node.object) + '[' + getNodeStr(node.property) + ']'
            }
            return getNodeStr(node.object) + '.' + getNodeStr(node.property)
        }
        case 'TryStatement':
            return 'try {}' + (node.handler ? ' catch() {}' : '') + (node.finalizer ? 'finally(){}' : '')
        case 'ThrowStatement':
            return 'throw ' + getNodeStr(node.argument)
        case 'ContinueStatement':
            return 'continue'
        case 'BreakStatement':
            return 'break'
        case 'NewExpression':
            return 'new ' + getNodeStr(node.callee) + `(${node.arguments.map(getNodeStr).join(', ')})`
        case 'ReturnStatement':
            return 'return ' + getNodeStr(node.argument)
        case 'SequenceExpression':
            return node.expressions.map(getNodeStr).join(',')
        case 'ArrayExpression':
            return '[' + node.elements.slice(0, 1).map(getNodeStr) + ']'
        case 'IfStatement':
            return 'if (' + getNodeStr(node.test) + ')' + getNodeStr(node.consequent) + (node.alternate ? 'else ' + getNodeStr(node.alternate) : '')
        case 'ConditionalExpression':
            return getNodeStr(node.test) + ' ? ' + getNodeStr(node.consequent) + ' : ' + getNodeStr(node.alternate)
        case 'UnaryExpression':
            return node.operator + ' ' + getNodeStr(node.argument)
        case 'UpdateExpression':
            return node.operator + getNodeStr(node.argument)
        case 'ObjectExpression':
            return '{' + node.properties.map(getNodeStr) + '}'
        case 'ObjectProperty':
            return getNodeStr(node.key) + ': ' + getNodeStr(node.value)
        case 'ForStatement':
            return `for(${getNodeStr(node.init)};${getNodeStr(node.test)};${getNodeStr(node.update)})` + getNodeStr(node.body)
        case 'ForInStatement':
            return `for(${getNodeStr(node.left)} in ${getNodeStr(node.right)})` + getNodeStr(node.body)
        case 'WhileStatement':
            return `while(${getNodeStr(node.test)})` + getNodeStr(node.body)
        case 'LabeledStatement':
            return `${node.label.name}:${getNodeStr(node.body)}`
        case 'SwitchStatement':
            return `switch(${getNodeStr(node.discriminant)}){}`
        case 'DoWhileStatement':
            return `do${getNodeStr(node.body)}while(${getNodeStr(node.test)})`
        default:
            return '___' + node.type + '___'
    }
}

function getObjectStr(value, showObjectValue) {
    try {
        let result
        if (value == this.globalScope) {
            result = 'window'
        }
        if (value === null) {
            result = 'null'
        } else if (typeof value === 'undefined') {
            result = 'undefined'
        } else if (typeof value === 'string') {
            result = '\'' + value.split('\n')[0] + '\''
        } else if (typeof value === 'number') {
            result = value + ''
        } else if (typeof value === 'boolean') {
            result = value + ''
        } else if (value instanceof Error) {
            result = value.message
        } else if (value instanceof TypeError) {
            result = value.message
        } else if (value instanceof Array) {
            const keys = Object.keys(value)
            result = '[' + keys.slice(0, 3).map(key => getObjectStr(value[key], false)) + `${keys.length > 3 ? ',...]' : ']'}`
        } else if (typeof value == 'object') {
            if (value.___func) {
                return `function ${value.___func.name}`
            }
            if (value.___Func) {
                return `function ${value.name}`
            }
            const keys = Object.keys(value)
            if (showObjectValue !== false) {
                result = '{' + keys.slice(0, 3).map(key => key + ':' + getObjectStr(value[key], false)).join(',') + `${keys.length > 3 ? ',...}' : '}'}`
            } else {
                result = '{' + keys.slice(0, 3).join(',') + `${keys.length > 3 ? ',...}' : '}'}`
            }
            if (value.__proto__) {
                const constructor = value.__proto__.constructor.name
                result = constructor + result
            }
        } else if (typeof value == 'function') {
            const isNative = value.toString().indexOf('native') !== -1
            if (isNative) {
                return value.toString()
            }
            if (value.___func) {
                result = 'function ' + value.___func.name + ':' + value.___func.___node.loc.start.line
            } else {
                result = 'function ' + value.name
            }
        } else if (typeof value == 'symbol') {
            result = value.toString()
        }
        return result
    } catch (e) {
        // console.log('getObjectStr', value)
        return 'getObjectStr error'
    }
}

function getValueOperator(oldValue, value, operator) {
    switch (operator) {
        case '=':
            return value
        case '+=':
            return oldValue += value
        case '-=':
            return oldValue -= value
        case '*=':
            return oldValue *= value
        case '/=':
            return oldValue /= value
        case '%=':
            return oldValue %= value
        case '>>=':
            return oldValue >>= value
        case '|=':
            return oldValue |= value
        case '&=':
            return oldValue &= value
        default:
            throw new Error(`not support update operator: ${operator}`)
    }
}


function setObjKey(obj, key, value, operator = '=') {
    if (obj === undefined || obj === null) {
        throw new Error('setObjKey obj is null')
    }
    if (obj.___Obj) {
        return obj.set(key, value, operator)
    } else {
        if (obj.___Func && key.startsWith('___')) {
            throw new Error('not support Func key')
        }
        if (isIgnoreObjKey(obj, key)) {
            return value
        }
        if (obj.createElement && obj.querySelectorAll) { // document
            // console.log('set', key, value)
        }
        if (typeof obj[key] == 'function') {
            if (obj[key] !== value) {
                // console.log('override ', key)
            } else {
                // console.log('override bind')
            }
        }
        if (value && value.___func) {
            value.___func.___setName(key)
        }
        return obj[key] = getValueOperator(obj[key], value, operator)
    }
}

function getObjValue(obj, key) {
    if (obj.___Obj) {
        return obj.get(key)
    }
    if (obj.createElement && obj.querySelectorAll) { // document
        // console.log('get', key, typeof obj[key])
        if (key === 'visibilityState') {
            return 'visible'
        }
    }
    return obj[key]
}

const fileMapper = {}

function writeFile(fileName, content) {
    let filePath = path.join(__dirname, fileName)
    let beforeContent = fileMapper[fileName] || ''
    const afterContent = beforeContent + content
    fs.writeFileSync(filePath, afterContent)
    fileMapper[fileName] = afterContent
}

module.exports = {
    writeFile,
    getElevationVariableList,
    getGlobalObj,
    getRunItem,
    getNodeStr,
    getAst,
    getNodeCode,
    isIgnoreObjKey,
    getObjectStr,
    getValueOperator,
    setObjKey,
    getObjValue,
}
