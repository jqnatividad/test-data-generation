## Test Data Generation

### Description
**PROBLEM**
</br>
In order to make test data represent production, (a.k.a. realistic) you need to perform one of the following:
+ load data from a production environment into the non-production environment, which requires ETL (e.g.: masking, obfuscation, etc.)
+ stand up a pre-loaded "profile" database that is randomly sampled, which requires preparing sample data from either another test data source 
or production environment (option #1 above)

**SOLUTION**
</br>
 By analyzing a sample data set (e.g.: 2016 Census of top 200 male first names), we are able to create an algorithm (profile) based on that data sample. 
 This algorithm can be easily stored (as a data file) and used to generate "realistic" test data as desired. 

---

### Archtecture 
![Architectural Diagram](./docs/img/test-data-generation-architecture.png "Architectural Diagram")



### Creating an Algorithm from Sample Data


---

## Continuous Integration
See [travis](./travis.yml) for detailed script

## Contributing

See [CONTRIBUTING](./CONTRIBUTING.md) for more information on technical details.

## License

test-data-generation is primarily distributed under the terms of the Apache License (Version 2.0).

See ![LICENSE-APACHE](.:LICENSE-APACHE "Apache License") for details.
