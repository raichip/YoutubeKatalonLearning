<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Experience the difference    Solutions_a281dc</name>
   <tag></tag>
   <elementGuidId>e255970f-41f1-4ef9-8ef5-c00d1b47db76</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>6f1297b6-82a9-4203-adbd-0da472c5bdff</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
		
			
				
				
					




  
  
  Experience the difference


  



  Solutions
  About Us
  Services
  Products
  Locations
  Admin Page
  

  



  home
  about
  contact


				
			
			
				
					
						
						
							


Welcome Cannia Putri

Account Services




  Open New Account
  Accounts Overview
  Transfer Funds
  Bill Pay
  Find Transactions
  Update Contact Info
  Request Loan
  Log Out

						
					
				
				
					



  
    Accounts Overview
    
      
        
          Account
          Balance*
          Available Amount
        
      
      

        
          14343
          $5000.00
          $5000.00
        

        
          Total
          $5000.00
           
        
      
      
        
          *Balance includes deposits that may be subject to holds
        
      
    

  

  



    var app = angular.module('OverviewAccountsApp', []);
    app.controller('OverviewAccountsCtrl', function ($scope, $http) {
        $scope.showOverview = true;
        $scope.showError = false;
        
        $http.get(&quot;services_proxy/bank/customers/&quot; + 12989 + &quot;/accounts&quot;, {timeout:30000})
            .then(function (response) {
                $scope.accounts = [];
                $scope.accounts = response.data;
                $scope.totalBalance = computeTotalBalance($scope.accounts);

                angular.forEach($scope.accounts, function(account) {
                    account.availableBalance = getAvailableBalance(account);
                });
            })
            .catch(function (response){
                showError(response);
            });

        function getAvailableBalance(account) {
            return account.balance &lt; 0 ? 0 : account.balance;
        }

        function computeTotalBalance(accounts) {
            var totalBalance = 0.0;
            angular.forEach(accounts, function(account) {
                totalBalance = totalBalance + parseFloat(account.balance, 10);
            });
            return totalBalance;
        }
        
        function showError(error) {
            $scope.showOverview = false;
            $scope.showError = true;
            var status = error.status > 0 ? error.status : &quot;timeout&quot;;
            var data = error.data ? error.data : &quot;Server timeout&quot;
            console.error(&quot;Server returned &quot; + status + &quot;: &quot; + data);
        }

    });

	app.filter('commaLess', function() {
		return function(input) {
			return (input) ? input.toString().trim().replace(&quot;,&quot;,&quot;&quot;) : null;
		};
	});

				
			
		
		



  
    
      Home| 
      About Us| 
      Services| 
      Products| 
      Locations| 
      Forum| 
      Site Map| 
      Contact Us
    
    © Parasoft. All rights reserved.
    
      Visit us at:
      www.parasoft.com
    
  

	

/html[1]/body[1]</value>
      <webElementGuid>e246e3c6-fd84-41b4-b128-2166abfb2697</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>e74ea3e6-ebf1-40b0-b1b6-30a576da57e2</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>7e0d4013-b16f-4e20-ab66-b09c259f1718</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
		
			
				
				
					




  
  
  Experience the difference


  



  Solutions
  About Us
  Services
  Products
  Locations
  Admin Page
  

  



  home
  about
  contact


				
			
			
				
					
						
						
							


Welcome Cannia Putri

Account Services




  Open New Account
  Accounts Overview
  Transfer Funds
  Bill Pay
  Find Transactions
  Update Contact Info
  Request Loan
  Log Out

						
					
				
				
					



  
    Accounts Overview
    
      
        
          Account
          Balance*
          Available Amount
        
      
      

        
          14343
          $5000.00
          $5000.00
        

        
          Total
          $5000.00
           
        
      
      
        
          *Balance includes deposits that may be subject to holds
        
      
    

  

  



    var app = angular.module(&quot; , &quot;'&quot; , &quot;OverviewAccountsApp&quot; , &quot;'&quot; , &quot;, []);
    app.controller(&quot; , &quot;'&quot; , &quot;OverviewAccountsCtrl&quot; , &quot;'&quot; , &quot;, function ($scope, $http) {
        $scope.showOverview = true;
        $scope.showError = false;
        
        $http.get(&quot;services_proxy/bank/customers/&quot; + 12989 + &quot;/accounts&quot;, {timeout:30000})
            .then(function (response) {
                $scope.accounts = [];
                $scope.accounts = response.data;
                $scope.totalBalance = computeTotalBalance($scope.accounts);

                angular.forEach($scope.accounts, function(account) {
                    account.availableBalance = getAvailableBalance(account);
                });
            })
            .catch(function (response){
                showError(response);
            });

        function getAvailableBalance(account) {
            return account.balance &lt; 0 ? 0 : account.balance;
        }

        function computeTotalBalance(accounts) {
            var totalBalance = 0.0;
            angular.forEach(accounts, function(account) {
                totalBalance = totalBalance + parseFloat(account.balance, 10);
            });
            return totalBalance;
        }
        
        function showError(error) {
            $scope.showOverview = false;
            $scope.showError = true;
            var status = error.status > 0 ? error.status : &quot;timeout&quot;;
            var data = error.data ? error.data : &quot;Server timeout&quot;
            console.error(&quot;Server returned &quot; + status + &quot;: &quot; + data);
        }

    });

	app.filter(&quot; , &quot;'&quot; , &quot;commaLess&quot; , &quot;'&quot; , &quot;, function() {
		return function(input) {
			return (input) ? input.toString().trim().replace(&quot;,&quot;,&quot;&quot;) : null;
		};
	});

				
			
		
		



  
    
      Home| 
      About Us| 
      Services| 
      Products| 
      Locations| 
      Forum| 
      Site Map| 
      Contact Us
    
    © Parasoft. All rights reserved.
    
      Visit us at:
      www.parasoft.com
    
  

	

/html[1]/body[1]&quot;) or . = concat(&quot;
		
			
				
				
					




  
  
  Experience the difference


  



  Solutions
  About Us
  Services
  Products
  Locations
  Admin Page
  

  



  home
  about
  contact


				
			
			
				
					
						
						
							


Welcome Cannia Putri

Account Services




  Open New Account
  Accounts Overview
  Transfer Funds
  Bill Pay
  Find Transactions
  Update Contact Info
  Request Loan
  Log Out

						
					
				
				
					



  
    Accounts Overview
    
      
        
          Account
          Balance*
          Available Amount
        
      
      

        
          14343
          $5000.00
          $5000.00
        

        
          Total
          $5000.00
           
        
      
      
        
          *Balance includes deposits that may be subject to holds
        
      
    

  

  



    var app = angular.module(&quot; , &quot;'&quot; , &quot;OverviewAccountsApp&quot; , &quot;'&quot; , &quot;, []);
    app.controller(&quot; , &quot;'&quot; , &quot;OverviewAccountsCtrl&quot; , &quot;'&quot; , &quot;, function ($scope, $http) {
        $scope.showOverview = true;
        $scope.showError = false;
        
        $http.get(&quot;services_proxy/bank/customers/&quot; + 12989 + &quot;/accounts&quot;, {timeout:30000})
            .then(function (response) {
                $scope.accounts = [];
                $scope.accounts = response.data;
                $scope.totalBalance = computeTotalBalance($scope.accounts);

                angular.forEach($scope.accounts, function(account) {
                    account.availableBalance = getAvailableBalance(account);
                });
            })
            .catch(function (response){
                showError(response);
            });

        function getAvailableBalance(account) {
            return account.balance &lt; 0 ? 0 : account.balance;
        }

        function computeTotalBalance(accounts) {
            var totalBalance = 0.0;
            angular.forEach(accounts, function(account) {
                totalBalance = totalBalance + parseFloat(account.balance, 10);
            });
            return totalBalance;
        }
        
        function showError(error) {
            $scope.showOverview = false;
            $scope.showError = true;
            var status = error.status > 0 ? error.status : &quot;timeout&quot;;
            var data = error.data ? error.data : &quot;Server timeout&quot;
            console.error(&quot;Server returned &quot; + status + &quot;: &quot; + data);
        }

    });

	app.filter(&quot; , &quot;'&quot; , &quot;commaLess&quot; , &quot;'&quot; , &quot;, function() {
		return function(input) {
			return (input) ? input.toString().trim().replace(&quot;,&quot;,&quot;&quot;) : null;
		};
	});

				
			
		
		



  
    
      Home| 
      About Us| 
      Services| 
      Products| 
      Locations| 
      Forum| 
      Site Map| 
      Contact Us
    
    © Parasoft. All rights reserved.
    
      Visit us at:
      www.parasoft.com
    
  

	

/html[1]/body[1]&quot;))]</value>
      <webElementGuid>d508d8b5-0cfc-4f44-b7c5-0f40bea962b6</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
