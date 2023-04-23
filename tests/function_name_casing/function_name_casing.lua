local function foo() end
local function foo(bar) end
local function fOO(bAR) end
local function fOo(bAr) end
local function Foo() end
local function Foo(bar) end

function X:a() end
function x:A() end

function X.X:a() end
function x.x:A() end

function X.a() end
function x.A() end

function x.x.a() end
function X.X.A() end

a()
a.a()
A()
A.A()
