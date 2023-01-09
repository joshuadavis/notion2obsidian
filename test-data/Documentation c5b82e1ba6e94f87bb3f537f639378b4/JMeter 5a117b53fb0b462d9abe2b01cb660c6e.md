# JMeter

# Initialization Step

Often you will want to make some REST calls to set up data for the test.   This is done by creating a 'set up thread'.   This is available in the thread group sub-menu.   With this, you can create a single thread that will execute before the main testing thread group.

1. Right click on the test plan, select *Add*→*Threads (Users)*
2. Select *setUp Thread Group.*   The number of threads should be set to 1 in most cases.
3. Add HTTP requests: Right click on the new setUp Thread Group.   *Select Add→Sampler→HTTP Request*

One common use case is to create some objects if they don't already exist

- [ ]  TODO: Write about how to do this.

# Main Test Thread Group

# Passing Command Line Parameters to a Test

1. Reference the parameter in the test configuration with:
`${__P(<variable name>,<default value>)}`
2. Set the parameter on the command line:
`jmeter -n -t <some test plan.jmx> -J<variable name>=<value>`

[Jmeter - command line script execution with arguments](https://mkbansal.wordpress.com/2012/08/01/jmeter-command-line-script-execution-with-arguments/)