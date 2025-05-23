<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>申請轉移憑證</name>
   <tag></tag>
   <elementGuidId>2a4a5cee-85a2-408f-a4d7-3472a7019493</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;origin_domain\&quot;: ${DOMAIN},\n  \&quot;domain\&quot;: \&quot;qatest${RANDOMNUM}.com\&quot;,\n  \&quot;site_group\&quot;: \&quot;dctest\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>de1fa1be-0a56-498e-919c-3226aba27715</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-api-key</name>
      <type>Main</type>
      <value>${PF_KEY}</value>
      <webElementGuid>f2a39685-3f5a-4992-adc6-14a6d269a745</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f8600913-f781-4cd3-a385-f80feaecdcee</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.7.4</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${DEV}/workflow_api/pf/application/reuse_certificate</restUrl>
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
      <id>ad07be1e-afd6-4f2d-8ed0-003e9e2fd4a4</id>
      <masked>false</masked>
      <name>DEV</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PF_KEY</defaultValue>
      <description></description>
      <id>5f34dd30-c4c2-4d13-a0c2-f75af8352d97</id>
      <masked>false</masked>
      <name>PF_KEY</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DOMAIN</defaultValue>
      <description></description>
      <id>c52d0969-f653-432f-9db3-6d76011669b3</id>
      <masked>false</masked>
      <name>DOMAIN</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RANDOMNUM</defaultValue>
      <description></description>
      <id>1a1af4f3-0c2a-4dd4-bcfa-2a8cee94d389</id>
      <masked>false</masked>
      <name>RANDOMNUM</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
