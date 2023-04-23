local function Foo() end
local function foo() end

local function Bar() end
local function bar() end

local function Baz() end
local function baz() end

function x.Foo() end
function x.foo() end

function x.Bar() end
function x.bar() end

function x.Baz() end
function x.baz() end

function x:Foo() end
function x:foo() end

function x:Bar() end
function x:bar() end

function x:Baz() end
function x:baz() end

Foo()
foo()
Bar()
bar()
Baz()
baz()
