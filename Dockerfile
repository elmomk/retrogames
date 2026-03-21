FROM busybox:1.37

RUN adduser -D -u 1000 app
COPY web/ /srv/www/
RUN chown -R app:app /srv/www

USER app
EXPOSE 8080

CMD ["busybox", "httpd", "-f", "-p", "8080", "-h", "/srv/www"]
