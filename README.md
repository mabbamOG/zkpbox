# zkpbox
A sandbox for Zero Knowledge proofs and their applications, written from scratch in pure Rust.

OBJECTIVES:
1. MODULARITY: For an economy to enable decentralised research and development, the average tinkerer needs to be able to easily modify the end products to integrate his desired functionality. A cornerstone for this objective is to build products whose internal components are modular, such that they can be easily swapped out for a modified homemade version. This is the first necdessary objective for enabling decentralised development.
2. SIMPLICITY: In order to further encourage distributed R&D, the product should also host components which are themselves easily understandable. This is because, even though a complex but modular product gives users the freedom to quickly swap components for new ones, a simple and modular product also gives users the freedom to apply minimal changes directly to its existing components, drastically reducing the amount of research and development required by the user to further enhance the product.
3. BASIC PERFORMANCE: In order to further encourage distributed R&D, it's important that the end users are capable of fully making use of the end products with resources that are already readily available to them. Building products that are optimised for performance without taking into account basic available hardware hampers the overall goal of encouraging distributed development on top of said products. Performance is an obviously necessary goal such that end users can make use of said products in a practical setting, with minimal waste of their available resources, at least in comparison to commonly available alternatives.

COMPONENTS:
   1. POLYNOMIAL COMMITMENT SCHEMES: The modern modular design of proof systems relies on a core cryptographic primitive called polynomial commitment schemes.  Available commitment schemes will be:
      - FRI
   2. ARITHMETISATION PROTOCOLS: The modern modulara design of proof systems begins with an arithmetisation process, which is used to convert program execution constraints into special polynomial checks, which are then fed into polynomial commitment schemes for final validation. Available arithmetisation protocols will be:
      - STARK
   3. CIRCUITS: In order for proof systems to be meaningful to the general public, they require meaningful use-cases. The modern approach to implementing such use-cases is in the form of polynomial constraints, also called arithmetic circuits, which are fed into a proof system in order to generate or verify proofs for specific use-cases. Available use-cases will be:
      - Hashing
     
RESULTS:

