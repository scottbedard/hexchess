<?php

use Symfony\Component\Yaml\Yaml;

/*
|--------------------------------------------------------------------------
| Test Case
|--------------------------------------------------------------------------
|
| The closure you provide to your test functions is always bound to a specific PHPUnit test
| case class. By default, that class is "PHPUnit\Framework\TestCase". Of course, you may
| need to change it using the "pest()" function to bind a different classes or traits.
|
*/

// pest()->extend(Tests\TestCase::class)->in('Feature');

/*
|--------------------------------------------------------------------------
| Expectations
|--------------------------------------------------------------------------
|
| When you're writing tests, you often need to check that values meet certain conditions. The
| "expect()" function gives you access to a set of "expectations" methods that you can use
| to assert different things. Of course, you may extend the Expectation API at any time.
|
*/

// expect()->extend('toBeOne', function () {
//     return $this->toBe(1);
// });

/*
|--------------------------------------------------------------------------
| Functions
|--------------------------------------------------------------------------
|
| While Pest is very powerful out-of-the-box, you may have some testing code specific to your
| project that you don't want to repeat in every file. Here you can also expose helpers as
| global functions to help you to reduce the number of lines of code in your test files.
|
*/

/** die and dump */
function dd(...$vars)
{
    foreach ($vars as $var) {
        fwrite(STDERR, print_r($var, true));
    }

    fwrite(STDERR, PHP_EOL);

    die(1);
}

/** get json fixture */
function json(string $name)
{
    $path = realpath(__DIR__ . "/../../tests/{$name}.json");
    $json = file_get_contents($path);
    return json_decode($json, true);
}

/** test json fixtures */
function testJson(string $name, Closure $fn)
{
    $data = json($name);

    return describe($name, function () use ($data, $fn) {
        foreach ($data as $t) {
            test($t['description'], fn () => $fn($t));
        }
    });
}

/** get yaml fixture */
function yaml(string $name)
{
    $path = realpath(__DIR__ . "/../../yaml-tests/{$name}.yaml");
    $yaml = file_get_contents($path);
    return Yaml::parse($yaml);
}

/** test yaml fixtures */
function testYaml(string $name, Closure $fn, string $primaryKey = 'description')
{
    $data = yaml($name);

    return describe($name, function () use ($data, $fn, $primaryKey) {
        foreach ($data as $t) {
            test($t[$primaryKey], fn () => $fn($t));
        }
    });
}