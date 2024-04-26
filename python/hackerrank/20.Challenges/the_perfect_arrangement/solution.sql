SELECT CUSTOMER.ID,
       CUSTOMER.FIRST_NAME || ' ' || CUSTOMER.LAST_NAME AS COMBINED_NAME
FROM CUSTOMER
WHERE LENGTH(CUSTOMER.FIRST_NAME || CUSTOMER.LAST_NAME) < 12
ORDER BY LENGTH(CUSTOMER.FIRST_NAME || CUSTOMER.LAST_NAME),
         COMBINED_NAME,
         CUSTOMER.ID;
