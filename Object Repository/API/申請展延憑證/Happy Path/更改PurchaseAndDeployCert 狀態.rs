<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>更改PurchaseAndDeployCert 狀態</name>
   <tag></tag>
   <elementGuidId>fbadbea0-ee0c-438f-ac09-cde2212f8e0a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;:${PurchaseAndDeployCert_job_id},\n    \&quot;status\&quot;:\&quot;success\&quot;\n    }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e3794d7f-2157-4ecd-90ae-eb85b2e84e17</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-API-Key</name>
      <type>Main</type>
      <value>${ADM_KEY}</value>
      <webElementGuid>60b67bf8-ff63-4c0f-b24c-7e757b50f279</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>eee44c8a-62a8-4ad7-808b-27466870ad86</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.7.4</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${DEV}/workflow_api/adm/jobs/status</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.DEV</defaultValue>
      <description></description>
      <id>554ff692-637a-460b-94e1-d249a2cf6904</id>
      <masked>false</masked>
      <name>DEV</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ADM_KEY</defaultValue>
      <description></description>
      <id>3c698202-415a-40e6-ba38-10326ec34594</id>
      <masked>false</masked>
      <name>ADM_KEY</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PurchaseAndDeployCert_job_id</defaultValue>
      <description></description>
      <id>04865eee-64ec-4595-bb2e-32f17da18520</id>
      <masked>false</masked>
      <name>PurchaseAndDeployCert_job_id</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
